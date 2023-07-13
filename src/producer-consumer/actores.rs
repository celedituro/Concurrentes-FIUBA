use std::{thread, time::Duration};

use actix::{Actor, Context, Handler, Message, System};
use rand::Rng;

#[derive(Message)]
#[rtype(result = "u32")]
struct Producir();

#[derive(Message)]
#[rtype(result = "()")]
struct Consumir(u32);

#[derive(Message)]
#[rtype(result = "u32")]
struct ConsultarAcumulado();

struct Productor {
}

struct Consumidor {
	acumulado: u32
}

impl Actor for Productor {
    type Context = Context<Self>; 
}

impl Actor for Consumidor {
    type Context = Context<Self>; 
}

impl Handler<Producir> for Productor {
    type Result = u32;

    fn handle(&mut self, _msg: Producir, _ctx: &mut <Productor as Actor>::Context) -> Self::Result  {
		println!("[PRODUCTOR]: pruduciendo");
		num_aleatorio()
    }
}

fn num_aleatorio() -> u32 {
    let mut rng = rand::thread_rng();
    let num: u32 = rng.gen_range(0..=10);
    num
}

impl Handler<Consumir> for Consumidor {
	type Result = ();
    fn handle(&mut self, msg: Consumir, _ctx: &mut <Consumidor as Actor>::Context) -> ()  {
		println!("[CONSUMIDOR]: consumiendo");
		self.acumulado += msg.0;
    }
}

impl Handler<ConsultarAcumulado> for Consumidor {
	type Result = u32;
    fn handle(&mut self, _msg: ConsultarAcumulado, _ctx: &mut <Consumidor as Actor>::Context) -> Self::Result  {
		println!("[CONSUMIDOR]: obteniendo acumulado");
		self.acumulado
    }
}

fn main() {
	System::new().block_on(async {

		let productor = Productor {}.start();
		let consumidor = Consumidor { acumulado: 0 }.start();
		
		loop {
            thread::sleep(Duration::from_millis(3000));
            let producto = productor.send(Producir()).await.unwrap();
            consumidor.send(Consumir(producto)).await.unwrap();
            let acumulado = consumidor.send(ConsultarAcumulado()).await.unwrap();

            println!("Acumulado: {:?} ", acumulado);
        }
    })
}
