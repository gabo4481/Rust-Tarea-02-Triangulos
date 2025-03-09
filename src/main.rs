use std::{fs::OpenOptions, io::{self, Write}};

fn main() {
    let mut coordenadas: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut lectura: String = String::new();
    let mut areas: Vec<f64> = Vec::new();
    let cantidad: i32;

    println!("Ingresa la cantidad de triangulos a calcular");
    io::stdin().read_line(&mut lectura).ok().expect("Error en la lectura");
    cantidad = lectura.trim().parse().ok().expect("Error en la conversion");
    for contador in 1..cantidad+1 {
        println!("Triangulo {}",contador);
        let (area,puntos) = calculo_area();
        areas.push(area);
        coordenadas.push(puntos);
    }
    mostrar_resultados(areas,coordenadas);
    
    
}

fn pedir_coordenadas () -> Vec<(f64,f64)> {
    let mut entrada: String = String::new();
    let mut puntos: Vec<(f64,f64)> = Vec::new();
    for contador in 1..3+1 {
        println!("Ingresa la coordenada X del vertice {}",contador);
        io::stdin().read_line(&mut entrada).ok().expect("Error en la lectura"); 
        let x = entrada.trim().parse().ok().expect("Error en la conversion");
        entrada.clear();
        println!("Ingresa la coordenada Y del vertice {}",contador);
        io::stdin().read_line(&mut entrada).ok().expect("Error en la lectura");
        let y = entrada.trim().parse().ok().expect("Error en la conversion");
        entrada.clear();
        puntos.push((x,y));
    }
    return puntos;
}

fn calculo_distancia(punto1: (f64,f64), punto2: (f64,f64)) -> f64 {
    let (x1,y1) = punto1;
    let (x2,y2) = punto2;
    let distancia = ((x2 - x1).powi(2) + (y2 - y1).powi(2)) as f64;
    return distancia.sqrt();
}

fn calculo_area() -> (f64,Vec<(f64,f64)>){
    let puntos = pedir_coordenadas();
    let d12 = calculo_distancia(puntos[0], puntos[1]);
    let d13 = calculo_distancia(puntos[0], puntos[2]);
    let d23 = calculo_distancia(puntos[1], puntos[2]);
    let s = (d12 + d13 + d23) / 2.0;
    let area = (s * (s - d12) * (s - d13) * (s - d23)).sqrt();
    return (area,puntos);
}

fn mostrar_resultados(areas: Vec<f64>,coordenadas: Vec<Vec<(f64, f64)>>) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("triangulos.txt").unwrap();

    writeln!(file,"Vertice 1\tVertice 2\tVertice 3\tArea").unwrap();
    for (_,(area,puntos)) in areas.iter().zip(coordenadas.iter()).enumerate(){
        writeln!(
            file,
            "({:.2?},{:.2?})\t({:.2?},{:.2?})\t({:.2?},{:.2?})\t{:.2?}",
            puntos[0].0,puntos[0].1,
            puntos[1].0,puntos[1].1,
            puntos[2].0,puntos[2].1
            ,area).unwrap();
    }
    
    
    
}
