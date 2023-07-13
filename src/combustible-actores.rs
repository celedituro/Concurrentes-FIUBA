use actix::{Actor, Context, Handler, Message, System};
use rand::Rng;

#[derive(Message)]
#[rtype(result = "u32")]
struct Consulta(u32, u32, u32);

struct Combustible {
}

impl Actor for Combustible {
    type Context = Context<Self>; 
}

impl Handler<Consulta> for Combustible {
    type Result = u32;

    fn handle(&mut self, msg: Consulta, _ctx: &mut <Combustible as Actor>::Context) -> Self::Result  {
        calcular_combustible( msg.0, msg.1, msg.2)
    }
}

fn calcular_combustible(posicion_acelerador: u32, pendiente_camino: u32, presion_aceite: u32) -> u32{
    let max = posicion_acelerador + pendiente_camino + presion_aceite;
    let mut rng = rand::thread_rng();
    let combustible: u32 = rng.gen_range(0..=max);
    
    combustible
}

fn main() {
    System::new().block_on(async {
        let addr = Combustible { }.start();
        let result = addr.send(Consulta(10, 20, 30)).await.unwrap();
    
        println!("Cantidad de combustible: {}", result);
    
        System::current().stop();
    })
}
