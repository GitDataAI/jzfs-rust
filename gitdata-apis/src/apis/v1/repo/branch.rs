use actix_session::Session;
use actix_web::Responder;
use actix_web::web;
use gitdata::model::repository::commit;

use crate::apis::app_writer::AppWrite;
use crate::service::repository::branchs::BranchCreateParma;
use crate::service::repository::branchs::BranchUpdateParma;
use crate::service::repository::branchs::BranchsParma;
use crate::service::users::info::UsersInfoReplay;

pub async fn api_v1_repo_branchs(
    path : web::Path<(String, String)>,
    session : Session,
    app_state : web::Data<crate::service::AppState>,
) -> impl Responder {
    let repo = match app_state
        .repo_owner_name(path.0.clone(), path.1.clone())
        .await
    {
        Ok(repo) => repo,
        Err(_) => {
            return AppWrite::<Vec<BranchsParma>>::not_found("Repository Not Found".to_string());
        }
    };
    if repo.visible {
        let branchs = match app_state.repo_branchs(repo).await {
            Ok(branchs) => branchs,
            Err(_) => {
                return AppWrite::<Vec<BranchsParma>>::not_found("Branch Not Found".to_string());
            }
        };
        return AppWrite::ok(branchs);
    } else {
        if let Ok(user) = UsersInfoReplay::from_session(session) {
            if user.uid == repo.owner_uid {
                let branchs = match app_state.repo_branchs(repo).await {
                    Ok(branchs) => branchs,
                    Err(_) => {
                        return AppWrite::<Vec<BranchsParma>>::not_found(
                            "Branch Not Found".to_string(),
                        );
                    }
                };
                return AppWrite::ok(branchs);
            } else {
                if user.member.contains(&user.uid) {
                    let branchs = match app_state.repo_branchs(repo).await {
                        Ok(branchs) => branchs,
                        Err(_) => {
                            return AppWrite::<Vec<BranchsParma>>::not_found(
                                "Branch Not Found".to_string(),
                            );
                        }
                    };
                    return AppWrite::ok(branchs);
                }
            }
        }
    }
    AppWrite::<Vec<BranchsParma>>::not_found("Branch Not Found".to_string())
}

pub async fn api_v1_repo_branch_info(
    path : web::Path<(String, String, String)>,
    session : Session,
    app_state : web::Data<crate::service::AppState>,
) -> impl Responder {
    let repo = match app_state
        .repo_owner_name(path.0.clone(), path.1.clone())
        .await
    {
        Ok(repo) => repo,
        Err(_) => return AppWrite::<BranchsParma>::not_found("Repository Not Found".to_string()),
    };
    if repo.visible {
        let branchs = match app_state.repo_branchs(repo).await {
            Ok(branchs) => branchs,
            Err(_) => return AppWrite::<BranchsParma>::not_found("Branch Not Found".to_string()),
        };
        for b in branchs {
            if b.name == path.2 {
                return AppWrite::ok(b);
            }
        }
    } else {
        if let Ok(user) = UsersInfoReplay::from_session(session) {
            if user.uid == repo.owner_uid {
                let branchs = match app_state.repo_branchs(repo).await {
                    Ok(branchs) => branchs,
                    Err(_) => {
                        return AppWrite::<BranchsParma>::not_found("Branch Not Found".to_string());
                    }
                };
                for b in branchs {
                    if b.name == path.2 {
                        return AppWrite::ok(b);
                    }
                }
            }
        }
    }
    AppWrite::<BranchsParma>::not_found("Branch Not Found".to_string())
}

pub async fn api_v1_branch_activity(
    path : web::Path<(String, String, String)>,
    app_state : web::Data<crate::service::AppState>,
) -> impl Responder {
    let repo = match app_state
        .repo_owner_name(path.0.clone(), path.1.clone())
        .await
    {
        Ok(repo) => repo,
        Err(_) => {
            return AppWrite::<Vec<commit::Model>>::not_found("Repository Not Found".to_string());
        }
    };
    let branchs = match app_state.repo_branchs(repo.clone()).await {
        Ok(branchs) => branchs,
        Err(_) => return AppWrite::not_found("Branch Not Found".to_string()),
    };
    for b in branchs {
        if b.name == path.2 {
            return match app_state.repo_branch_activity(repo, b.name).await {
                Ok(activity) => AppWrite::ok(activity),
                Err(_) => AppWrite::not_found("Branch Not Found".to_string()),
            };
        }
    }
    AppWrite::not_found("Branch Not Found".to_string())
}

pub async fn api_v1_repo_branch_create(
    path : web::Path<(String, String)>,
    session : Session,
    app_state : web::Data<crate::service::AppState>,
    param : web::Json<BranchCreateParma>,
) -> impl Responder {
    let repo = match app_state
        .repo_owner_name(path.0.clone(), path.1.clone())
        .await
    {
        Ok(repo) => repo,
        Err(_) => return AppWrite::<BranchsParma>::not_found("Repository Not Found".to_string()),
    };
    let user = match UsersInfoReplay::from_session(session) {
        Ok(user) => user,
        Err(_) => return AppWrite::<BranchsParma>::unauthorized("User Not Found".to_string()),
    };
    if user.uid == repo.owner_uid {
        return match app_state.repo_branch_create(repo, param.into_inner()).await {
            Ok(_) => AppWrite::ok_msg("Branch Create Success".to_string()),
            Err(_) => AppWrite::<BranchsParma>::not_found("Branch Not Found".to_string()),
        };
    } else {
        let member = match app_state.repo_member(repo.clone()).await {
            Ok(member) => member,
            Err(_) => return AppWrite::<BranchsParma>::not_found("Branch Not Found".to_string()),
        };
        if member.iter().any(|x| x.uid == user.uid) {
            return match app_state.repo_branch_create(repo, param.into_inner()).await {
                Ok(_) => AppWrite::ok_msg("Branch Create Success".to_string()),
                Err(_) => AppWrite::<BranchsParma>::not_found("Branch Not Found".to_string()),
            };
        }
    }
    return AppWrite::<BranchsParma>::unauthorized("User Not Found".to_string());
}

pub async fn api_v1_repo_branch_rename(
    path : web::Path<(String, String, String)>,
    session : Session,
    app_state : web::Data<crate::service::AppState>,
    param : web::Json<BranchUpdateParma>,
) -> impl Responder {
    let repo = match app_state
        .repo_owner_name(path.0.clone(), path.1.clone())
        .await
    {
        Ok(repo) => repo,
        Err(_) => return AppWrite::<BranchsParma>::not_found("Repository Not Found".to_string()),
    };
    let user = match UsersInfoReplay::from_session(session) {
        Ok(user) => user,
        Err(_) => return AppWrite::<BranchsParma>::unauthorized("User Not Found".to_string()),
    };
    if user.uid == repo.owner_uid {
        return match app_state.repo_branch_update(repo, param.into_inner()).await {
            Ok(_) => AppWrite::ok_msg("Branch Rename Success".to_string()),
            Err(_) => AppWrite::<BranchsParma>::not_found("Branch Not Found".to_string()),
        };
    } else {
        let member = match app_state.repo_member(repo.clone()).await {
            Ok(member) => member,
            Err(_) => return AppWrite::<BranchsParma>::not_found("Branch Not Found".to_string()),
        };
        if member.iter().any(|x| x.uid == user.uid) {
            return match app_state.repo_branch_update(repo, param.into_inner()).await {
                Ok(_) => AppWrite::ok_msg("Branch Rename Success".to_string()),
                Err(_) => AppWrite::<BranchsParma>::not_found("Branch Not Found".to_string()),
            };
        }
    }
    AppWrite::<BranchsParma>::unauthorized("User Not Found".to_string())
}

pub async fn api_v1_repo_branch_delete(
    path : web::Path<(String, String, String)>,
    session : Session,
    app_state : web::Data<crate::service::AppState>,
    param : web::Json<BranchUpdateParma>,
) -> impl Responder {
    let repo = match app_state
        .repo_owner_name(path.0.clone(), path.1.clone())
        .await
    {
        Ok(repo) => repo,
        Err(_) => return AppWrite::<BranchsParma>::not_found("Repository Not Found".to_string()),
    };
    let user = match UsersInfoReplay::from_session(session) {
        Ok(user) => user,
        Err(_) => return AppWrite::<BranchsParma>::unauthorized("User Not Found".to_string()),
    };
    if user.uid == repo.owner_uid {
        match app_state.repo_branch_delete(repo, param.into_inner()).await {
            Ok(_) => AppWrite::ok_msg("Branch Delete Success".to_string()),
            Err(_) => AppWrite::<BranchsParma>::not_found("Branch Not Found".to_string()),
        }
    } else {
        let member = match app_state.repo_member(repo.clone()).await {
            Ok(member) => member,
            Err(_) => return AppWrite::<BranchsParma>::not_found("Branch Not Found".to_string()),
        };
        if member.iter().any(|x| x.uid == user.uid) {
            return match app_state.repo_branch_delete(repo, param.into_inner()).await {
                Ok(_) => AppWrite::ok_msg("Branch Delete Success".to_string()),
                Err(_) => AppWrite::<BranchsParma>::not_found("Branch Not Found".to_string()),
            };
        }
        AppWrite::<BranchsParma>::unauthorized("User Not Found".to_string())
    }
}
