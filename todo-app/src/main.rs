use std::{
	io,
	io::{ BufRead, Error, Read, Write },
	fs::{ OpenOptions, write }
};

#[derive(Debug)]
struct Todo {
	task: String,
	done: bool
}

impl Todo {
	fn create(task: String, done: bool) -> Todo {
		Todo { task, done }
	}

	fn save(&self) -> Result<(), Error> {
		let task = format!("{}{}{}", self.task, ":", self.done);
		let mut file = OpenOptions::new()
																		.write(true)
																		.append(true)
																		.open("todo.txt")
																		.expect("Algo ha fallado al abrir el archivo");

		writeln!(file, "{}", task).expect("Algo ha fallado al guardar el texto");
		Ok(())
	}
}

fn main() {
	let mut todo: Vec<Todo> = all_todo().expect("Ha ocurrido al obtener la lista de tareas");
	println!("Tareas: {:?}", todo);

	loop {
		let stdin = io::stdin();

		println!("Qué acciónn deseas realizar?");
		let action = stdin.lock().lines().next().unwrap().unwrap();

		if action == "break" {
			break
		};

		println!("Ingresa el nombre de la tarea");
		let task = stdin.lock().lines().next().unwrap().unwrap();
		
		match action.as_ref() {
			"show" => show_todo(&mut todo, task),
			"create" => create_todo(&mut todo, task),
			"complete" => complete_todo(&mut todo, task),
			"delete" => delete_todo(&mut todo, task),
			_ => println!("La opci?????�n es invalida :(")
		}
	}
}

fn all_todo() -> Result<Vec<Todo>, Error> {
	let mut file = OpenOptions::new()
																	.write(true)
																	.create(true)
																	.read(true)
																	.open("todo.txt")
																	.expect("Ha ocurrido un error al intentar abrir el archivo");
	let mut body = String::new();
	file.read_to_string(&mut body).expect("No se ha podido leer el archivo.");
	let mut list: Vec<Todo> = Vec::new();
	for line in body.lines() {
		// task:false
		let task = line.split(':').collect::<Vec<&str>>();
		list.push(
			Todo::create(task[0].to_string(), task[1].parse().unwrap())
		)
	}

	Ok(list)
}

fn show_todo(todo: &mut Vec<Todo>, status_task: String) {
	println!("Lista de tareas por hacer \n");
	for task in todo {
		let status = if !task.done { "Por hacer" } else { "Completada" };
		if status_task == "all" {
			println!("{} - {}", task.task, status);
		} else if status_task == "completed" && task.done {
			println!("{} - {}", task.task, status);
		} else if status_task == "todo" && !task.done {
			println!("{} - {}", task.task, status);
		}
	}
}

fn create_todo(todo: &mut Vec<Todo>, task: String) {
	let todo_instance = Todo::create(task, false);
	match todo_instance.save() {
		Ok(_) => {
			todo.push(todo_instance);
			println!("La tarea ha sido guardada correctamente");
		}
		Err(error) => println!("Ha ocurrido un error --> {}", error)
	}
}

fn complete_todo(todo: &mut Vec<Todo>, arg_task: String) {
	let mut body = String::new();
	for task in todo {
		let task_done = if task.task == arg_task { true } else { task.done };
		let current_task = format!(
			"{}:{}\n", 
			task.task,
			task_done
		);
		task.done = task_done;
		body.push_str(&current_task);
	}

	match write("todo.txt", body) {
		Ok(_) => println!("Tarea ha sido marcada como completada"),
		Err(error) => println!("Ha ocurrido un error --> {}", error)
	}
}

fn delete_todo(todo: &mut Vec<Todo>, arg_task: String) {
	let mut body = String::new();
	for task in &mut todo.into_iter() {
		let current_task = format!(
			"{}:{}\n", 
			task.task,
			task.done
		);
		if task.task != arg_task {
			body.push_str(&current_task);
		}
	}

	todo.retain(|value| value.task != arg_task);

	match write("todo.txt", body) {
		Ok(_) => println!("Tarea ha sido eliminada con exito"),
		Err(error) => println!("Ha ocurrido un error --> {}", error)
	}
}