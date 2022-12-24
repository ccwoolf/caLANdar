use sqlx::PgPool;

use crate::{
    controllers::Error,
    repositories::{event, invitation},
    routes::events::{Event, EventCreate},
};

pub async fn get_all(pool: &PgPool) -> Result<Vec<Event>, Error> {
    // Return all events
    match event::index(pool).await {
        Ok(events) => {
            // Convert vector of Event structs to vector of EventsGetResponse structs
            Ok(events.into_iter().map(Event::from).collect())
        }
        Err(e) => Err(Error::ControllerError(format!(
            "Unable to get list of events due to: {}",
            e
        ))),
    }
}

pub async fn get_all_user(pool: &PgPool, user_email: String) -> Result<Vec<Event>, Error> {
    let invitation_filter_values = invitation::Filter {
        event_id: None,
        email: Some(user_email),
    };

    let invitations: Vec<i32> = match invitation::filter(pool, invitation_filter_values).await {
        Ok(invitations) => {
            // Return list of event IDs
            invitations
                .into_iter()
                .map(|invitation| invitation.event_id)
                .collect()
        }
        Err(e) => {
            return Err(Error::ControllerError(format!(
                "Unable to get user's invitations due to: {}",
                e
            )))
        }
    };

    // Build new EventFilter
    let event_filter_values = event::Filter {
        ids: Some(invitations),
    };
    // Return all events
    match event::filter(pool, event_filter_values).await {
        Ok(events) => {
            // Convert vector of Event structs to vector of EventsGetResponse structs
            Ok(events.into_iter().map(Event::from).collect())
        }
        Err(e) => Err(Error::ControllerError(format!(
            "Unable to get users's events due to: {}",
            e
        ))),
    }
}

pub async fn get(pool: &PgPool, id: i32) -> Result<Event, Error> {
    // Build new EventFilter
    let event_filter_values = event::Filter {
        ids: Some(vec![id]),
    };
    // Return all events
    match event::filter(pool, event_filter_values).await {
        Ok(events) => {
            // Check we have exactly one event
            match events.len() {
                1 => Ok(Event::from(events[0].clone())),
                0 => Err(Error::NoDataError("No events found".to_string())),
                _ => Err(Error::ControllerError(format!(
                    "Too many events returned for id {}",
                    id
                ))),
            }
        }
        Err(e) => Err(Error::ControllerError(format!(
            "Unable to get event due to: {}",
            e
        ))),
    }
}

pub async fn get_user(pool: &PgPool, id: i32, user_email: String) -> Result<Event, Error> {
    let invitation_filter_values = invitation::Filter {
        event_id: Some(id),
        email: Some(user_email),
    };

    let invitations: Vec<i32> = match invitation::filter(pool, invitation_filter_values).await {
        Ok(invitations) => {
            // Return list of event IDs
            invitations
                .into_iter()
                .map(|invitation| invitation.event_id)
                .collect()
        }
        Err(e) => {
            return Err(Error::ControllerError(format!(
                "Unable to get user's invitations due to: {}",
                e
            )))
        }
    };

    // Build new EventFilter
    let event_filter_values = event::Filter {
        ids: Some(invitations),
    };

    // Return all events
    match event::filter(pool, event_filter_values).await {
        Ok(events) => {
            // Check we have exactly one event
            match events.len() {
                1 => Ok(Event::from(events[0].clone())),
                0 => Err(Error::NoDataError("No events found".to_string())),
                _ => Err(Error::ControllerError(format!(
                    "Too many events returned for id {}",
                    id
                ))),
            }
        }
        Err(e) => Err(Error::ControllerError(format!(
            "Unable to get users's events due to: {}",
            e
        ))),
    }
}

pub async fn delete(pool: &PgPool, id: i32) -> Result<(), Error> {
    // Build new EventFilter
    let event_filter_values = event::Filter {
        ids: Some(vec![id]),
    };

    // Delete event
    match event::delete(pool, event_filter_values).await {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::ControllerError(format!(
            "Unable to delete event due to: {}",
            e
        ))),
    }
}

pub async fn create(pool: &PgPool, new_event: EventCreate) -> Result<Event, Error> {
    // Delete event
    match event::create(
        pool,
        new_event.title,
        new_event.description,
        new_event.time_begin,
        new_event.time_end,
    )
    .await
    {
        Ok(event) => Ok(Event::from(event)),
        Err(e) => Err(Error::ControllerError(format!(
            "Unable to create event due to: {}",
            e
        ))),
    }
}
