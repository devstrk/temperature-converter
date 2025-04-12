use colored::Colorize;
use std::io::stdin;

enum TemperatureUnit {
  Celsius,
  Fahrenheit,
}

impl TemperatureUnit {
  fn symbol(&self) -> &'static str {
    match self {
      TemperatureUnit::Celsius => "°C",
      TemperatureUnit::Fahrenheit => "°F",
    }
  }

  fn name(&self) -> &'static str {
    match self {
      TemperatureUnit::Celsius => "Цельсия",
      TemperatureUnit::Fahrenheit => "Фаренгейты",
    }
  }
}

struct ConversionsOption {
  key: &'static str,
  from_unit: TemperatureUnit,
  to_unit: TemperatureUnit,
  converter: fn(f64) -> f64,
}

impl ConversionsOption {
  fn description(&self) -> String {
    format!(
      "{} → {} ({} в {})",
      self.from_unit.symbol(),
      self.to_unit.symbol(),
      self.from_unit.name(),
      self.to_unit.name()
    )
  }
}

const OPTIONS: [ConversionsOption; 2] = [
  (ConversionsOption {
    key: "1",
    from_unit: TemperatureUnit::Celsius,
    to_unit: TemperatureUnit::Fahrenheit,
    converter: convert_to_fahrenheit,
  }),
  (ConversionsOption {
    key: "2",
    from_unit: TemperatureUnit::Fahrenheit,
    to_unit: TemperatureUnit::Celsius,
    converter: convert_to_celsius,
  }),
];

fn main() {
  println!(
    "\n{}",
    " Добро пожаловать в конвертер температур! "
      .bold()
      .black()
      .on_cyan()
  );

  loop {
    let input_temperature = get_valid_temperature();
    if !select_and_convert_temperature(input_temperature) {
      break;
    };
  }
}

fn get_valid_temperature() -> f64 {
  loop {
    let mut input: String = String::new();

    println!(
      "\n{}\n",
      "Введите значение температуры для конвертации".bold().cyan()
    );
    stdin()
      .read_line(&mut input)
      .expect("\nОшибка ввода, повторите попытку.");

    match input.trim().parse::<f64>() {
      Ok(value) => break value,
      Err(_) => {
        println!("\n{}", "Ошибка: введите число".red());
        continue;
      }
    }
  }
}

fn convert_to_celsius(fahrenheit: f64) -> f64 {
  (fahrenheit - 32.0) * 5.0 / 9.0
}

fn convert_to_fahrenheit(celsius: f64) -> f64 {
  (celsius * 9.0 / 5.0) + 32.0
}

fn print_menu() {
  println!("\n{}\n", "Конвертировать в ...".bold().cyan());

  for option in OPTIONS.iter() {
    println!("{}. {}", option.key, option.description());
  }
  println!("{}. Выход\n", OPTIONS.len() + 1);
}

fn print_result(input_temperature: f64, from: &str, converted_temperature: f64, to: &str) {
  println!("\n{}", "─".repeat(30).cyan());
  println!(
    "\n{:.2}{} = {}{}\n",
    input_temperature,
    from,
    format!("{:.2}", converted_temperature).bold().green(),
    to.bold().green()
  );
  println!("{}", "─".repeat(30).cyan());
}

fn select_and_convert_temperature(input_temperature: f64) -> bool {
  loop {
    print_menu();

    let mut choice = String::new();
    stdin()
      .read_line(&mut choice)
      .expect("\nОшибка ввода, повторите попытку.");

    if choice.trim() == (OPTIONS.len() + 1).to_string() {
      println!("\n{}\n", "До свидания!".bold().cyan());
      break false;
    };

    match OPTIONS
      .iter()
      .find(|ConversionsOption { key, .. }| key == &choice.trim())
    {
      Some(&ConversionsOption {
        ref from_unit,
        ref to_unit,
        converter,
        ..
      }) => {
        print_result(
          input_temperature,
          from_unit.symbol(),
          converter(input_temperature),
          to_unit.symbol(),
        );
        break true;
      }
      None => continue,
    }
  }
}
