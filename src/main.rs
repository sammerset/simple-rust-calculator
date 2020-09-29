#[macro_use]
extern crate nickel;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

// Nickel
use nickel::{Nickel, JsonBody, HttpRouter, MediaType};
use nickel::status::StatusCode;

#[derive(Serialize, Deserialize)]
struct Output {
  h: i32,
  k: f64
}

#[derive(Serialize, Deserialize)]
struct Input {
  a: bool,
  b: bool,
  c: bool,
  d: f64,
  e: i64,
  f: i64
}

// Predefined values
const PDEF_M: i32 = 10;
const PDEF_P: i32 = 20;
const PDEF_T: i32 = 30;

// basic
struct Calc {}

trait Expressions {
  fn m_expr(&self, data: Input) -> f64 {
    return data.d + (data.d * (data.e as f64) / (10 as f64));
  }

  fn p_expr(&self, data: Input) -> f64 {
    return data.d + (data.d * ((data.e as f64) - (data.f as f64)) / 25.5);
  }

  fn t_expr(&self, data: Input) -> f64 {
    return data.d - (data.d * (data.f as f64) / (30 as f64));
  }

  fn basic(&self, data: Input) -> Result<Output, &str> {
    if data.a && data.b && !data.c {
      return Ok(Output { h: PDEF_M, k: self.m_expr(data) });
    } else if data.a && data.b && data.c {
      return Ok(Output { h: PDEF_P, k: self.p_expr(data) });
    } else if !data.a && data.b && data.c {
      return Ok(Output { h: PDEF_T, k: self.t_expr(data) });
    } else {
      return Err("Undefined input data");
    }
  }

  fn result(&self, data: Input) -> Result<Output, &str> {
    return self.basic(data);
  }
}

impl Expressions for Calc {}

// Extentions
struct Calc1 {}
struct Calc2 {}

impl Expressions for Calc1 {
  fn p_expr(&self, data: Input) -> f64 {
    return (2 as f64) * data.d + (data.d * (data.e as f64) / (100 as f64));
  }
}

impl Expressions for Calc2 {
  fn m_expr(&self, data: Input) -> f64 {
    return (data.f as f64) + data.d + (data.d * (data.e as f64) / (100 as f64));
  }

  fn result(&self, data: Input) -> Result<Output, &str> {
    if data.a && data.b && !data.c {
      return Ok(Output { h: PDEF_T, k: self.t_expr(data) });
    } else if data.a && !data.b && data.c {
      return Ok(Output { h: PDEF_M, k: self.m_expr(data) });
    } else {
      return self.basic(data);
    }
  }
}

fn main() {
  let mut server = Nickel::new();

  server.post("/:version", middleware! { |request, mut response|
    let input = try_with!(response, {
      request.json_as::<Input>().map_err(|e| (StatusCode::BadRequest, e))
    });

    let version = request.param("version").unwrap();

    let result = match &version {
      &"1" => (Calc1 {}).result(input),
      &"2" => (Calc2 {}).result(input),
      &_ => (Calc {}).result(input)
    };

    response.set(MediaType::Json);

    serde_json::to_string(&result).unwrap()
  });

  server.listen("127.0.0.1:6767").unwrap();
}

