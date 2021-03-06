// Evelyn: Your personal assistant, project manager and calendar
// Copyright (C) 2017 Gregory Jensen
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use core::error_messages::{EvelynBaseError, EvelynCoreError};
use data;
use model::user::{CreateUserRequestModel, LogonUserRequestModel, LogonUserResponseModel, UserModel, SearchRequestModel, SearchResponseModel, SearchResultExternal};
use processing::ProcessorData;
use std::sync::Arc;
use uuid::Uuid;

pub fn create_user(
    model: CreateUserRequestModel,
    processor_data: Arc<ProcessorData>,
) -> Option<EvelynCoreError> {
    let user_id = Uuid::new_v4();

    let user_model = UserModel {
        user_id: format!("{}", user_id),
        user_name: model.user_name,
        email_address: model.email_address,
        password: model.password,
    };

    let ds = processor_data.data_store.clone();

    match data::user::find_user(&ds, &user_model.email_address) {
        Ok(user) => {
            if user.is_some() {
                Some(EvelynCoreError::WillNotCreateUserBecauseUserAlreadyExists(EvelynBaseError::NothingElse))
            }
            else {
                let error = data::user::insert_user(&ds, &user_model);
                if error.is_some() {
                    Some(EvelynCoreError::FailedToCreateUser(error.unwrap()))
                }
                else {
                    // There were no errors.
                    None
                }
            }
        },
        Err(e) => Some(EvelynCoreError::CannotCheckIfUserExistsSoWillNotCreateNewUser(e)),
    }
}

pub fn logon_user(
    model: LogonUserRequestModel,
    processor_data: Arc<ProcessorData>,
) -> Result<LogonUserResponseModel, EvelynCoreError> {
    let ds = processor_data.data_store.clone();

    match data::user::find_user(&ds, &model.email_address) {
        Ok(user) => {
            if user.is_some() {
                let user = user.unwrap();
                if user.password == model.password {
                    let token = processor_data
                        .token_service
                        .create_session_token(&processor_data.server_session_token, &user);

                    Ok(LogonUserResponseModel {
                           token: Some(token),
                           error: None,
                       })
                } else {
                    Err(EvelynCoreError::InvalidLogon(EvelynBaseError::NothingElse))
                }
            } else {
                Err(EvelynCoreError::InvalidLogon(EvelynBaseError::NothingElse))
            }
        },
        Err(e) => Err(EvelynCoreError::FailedToLogonUser(e)),
    }
}

pub fn search_for_users(
    model: SearchRequestModel,
    processor_data: Arc<ProcessorData>,
) -> Result<SearchResponseModel, EvelynCoreError> {
    let ds = processor_data.data_store.clone();

    match data::user::search_for_users(&ds, model.query) {
        Ok(search_results) => {
            Ok(SearchResponseModel {
                search_results: search_results.into_iter().map(|x| SearchResultExternal {
                    user_id: x.user_id,
                    user_name: x.user_name,
                }).collect(),
                error: None,
            })
        },
        Err(e) => Err(EvelynCoreError::FailedToSearchForUsers(e)),
    }
}
