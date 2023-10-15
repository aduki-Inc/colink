use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use std::env;
use bcrypt::{hash, verify};
use diesel::r2d2::{ConnectionManager, Pool};
use serde::{Deserialize, Serialize};

