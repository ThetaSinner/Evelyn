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

use chrono::prelude::*;

use core::error_messages::EvelynCoreError;
use data;
use model;
use processing::ProcessorData;
use std::cmp::Ordering;
use std::sync::Arc;
use uuid::Uuid;

pub fn create_simple_task(
    model: model::simple_task::CreateSimpleTaskRequestModel,
    session_token_model: model::SessionTokenModel,
    processor_data: Arc<ProcessorData>,
) -> Result<model::simple_task::CreateSimpleTaskResponseModel, EvelynCoreError> {
    let task_id = Uuid::new_v4();

    let simple_task_model = model::simple_task::SimpleTaskModel {
        user_id: session_token_model.user_id,
        task_id: format!("{}", task_id),
        title: model.title,
        description: model.description,
        due_date: model.due_date,
        completed: false,
    };

    let ds = processor_data.data_store.clone();

    match data::simple_task::insert_simple_task(&ds, &simple_task_model) {
        Some(e) => Err(EvelynCoreError::FailedToCreateSimpleTask(e)),
        None => {
            Ok(model::simple_task::CreateSimpleTaskResponseModel {
                   task_id: Some(format!("{}", task_id)),
                   error: None,
               })
        },
    }
}

pub fn lookup_simple_tasks(
    model: model::simple_task::LookupSimpleTaskRequestModel,
    session_token_model: model::SessionTokenModel,
    processor_data: Arc<ProcessorData>,
) -> Result<model::simple_task::LookupSimpleTaskResponseModel, EvelynCoreError> {
    let simple_task_lookup_model = model::simple_task::SimpleTaskLookupModel {
        user_id: session_token_model.user_id,
        limit: model.limit,
        show_completed: model.show_completed,
    };

    let ds = processor_data.data_store.clone();

    match data::simple_task::lookup_simple_tasks(&ds, &simple_task_lookup_model) {
        Ok(mut tasks) => {
            tasks.sort_by(|a, b| {
                let a_date = a.due_date.parse::<DateTime<Utc>>();
                let b_date = b.due_date.parse::<DateTime<Utc>>();

                // TODO unsafe
                if a_date.unwrap().eq(&b_date.unwrap()) {
                    if a.title < b.title {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                } else if a_date.unwrap().lt(&b_date.unwrap()) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });

            let mut filtered_tasks: Vec<model::simple_task::SimpleTaskModel> = Vec::new();
            for x in tasks {
                if simple_task_lookup_model.show_completed {
                    filtered_tasks.push(x);
                } else if !simple_task_lookup_model.show_completed && !x.completed {
                    filtered_tasks.push(x);
                }
            }

            if simple_task_lookup_model.limit > 0 {
                filtered_tasks.truncate(simple_task_lookup_model.limit as usize);
            }

            Ok(model::simple_task::LookupSimpleTaskResponseModel {
                   simple_tasks: filtered_tasks.into_iter().map(|x| {
                       model::simple_task::SimpleTaskExternalModel {
                           task_id: x.task_id,
                           title: x.title,
                           description: x.description,
                           due_date: x.due_date,
                           completed: x.completed,
                       }
                   }).collect(),
                   error: None,
            })
        },
        Err(e) => Err(EvelynCoreError::FailedToLookupSimpleTask(e)),
    }
}

pub fn update_simple_task(
    model: model::simple_task::UpdateSimpleTaskRequestModel,
    session_token_model: model::SessionTokenModel,
    processor_data: Arc<ProcessorData>,
) -> Option<EvelynCoreError> {
    let simple_task_update_model = model::simple_task::SimpleTaskUpdateModel {
        user_id: session_token_model.user_id,
        task_id: model.task_id,
        title: model.new_title,
        description: model.new_description,
        due_date: model.new_due_date,
        completed: model.new_completed,
    };

    let ds = processor_data.data_store.clone();

    match data::simple_task::update_simple_task(&ds, simple_task_update_model) {
        None => None,
        Some(e) => Some(EvelynCoreError::FailedToUpdateSimpleTask(e)),
    }
}

pub fn remove(
    model: model::simple_task::RemoveSimpleTaskRequestModel,
    processor_data: Arc<ProcessorData>,
) -> Option<EvelynCoreError> {
    let ds = processor_data.data_store.clone();

    match data::simple_task::remove(&ds, model.task_id) {
        None => None,
        Some(e) => Some(EvelynCoreError::FailedToRemoveSimpleTask(e)),
    }
}
