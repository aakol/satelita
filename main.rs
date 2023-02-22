


//use std::ptr::read_unaligned;
//use std::vec;
use std::iter::Iterator;
//use std::fs::File;
//use std::io::prelude::*;
use std::io;
use std::ops::Add;
//use std::str;
//use crate::io::stdout;
//use std::io::stdout;
//use std::io::Write;
//quse std::io::{stdin, stdout, Read, Write};
use std::path::Path;
use std::{env};
extern crate sdl2;

use sdl2::libc::VM_FLAGS_RETURN_4K_DATA_ADDR;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
//use sdl2::render::Canvas;
//use sdl2::render::Canvas;
use sdl2::render::WindowCanvas;
use std::slice::Windows;
//use std::ops::Deref;
use std::time::Duration;


fn wyswietl(rend: &mut WindowCanvas ,min_x: i64, max_x:i64 , x: i64, min_y: i64, max_y:i64 ,  y:i64 , max: i64 , min: i64 , wartosc: i64 , p_x: i32 , p_y: i32 , s: i64)
{
    
    let skala_x = (max_x  - min_x ) / s;
    let mut x_trajektoria = ((x - min_x ) / skala_x) as i32; 

    let skala_y = (max_y  - min_y ) / s;
    let mut y_trajektoria = ((y - min_y ) / skala_y) as i32; 
    
  
    let skala: f64 = ((max  - min ) / 250) as f64;
    let mut kolor = ((wartosc as f64 - min as f64 ) / skala ) as u8 ; 
    //print!("{}", kolor);
   
   

   
 
   
        //rend.set_draw_color(Color::RGB(skala.try_into().unwrap() , skala.try_into().unwrap() , skala.try_into().unwrap()));
        rend.set_draw_color(Color::RGB(kolor , kolor , 200));
        rend.draw_point(( x_trajektoria+p_x, y_trajektoria+p_y));
       // print!("{} {} {} {} {}  ", x,y,max,min,warosc);
    //cargo runrend.draw_line((x_trajektoria+p_x-10, y_trajektoria+p_y-10), (x_trajektoria+p_x+10, y_trajektoria+p_y+10));
    
    
    
}
fn mapa (mapka:  &mut WindowCanvas , min_x: i64, max_x:i64 , min_y: i64, max_y:i64 , p_x:i32 , p_y: i32 , s: i64)
{
let x = vec![53904392 ,54274773,54539562,54822208, 54602542,54775237, 54408190 ,53349402, 54454475, 54393592, 54075623,52682047, 52241663, 52057021,50396684, 49498888, 49004079, 50974411, 53903261];
let y = vec![14357171 ,16115853,16591172,18336274 ,18838754,18424547, 18660495 ,18951633 ,19642494 ,22940997 ,23523352,23886528, 23180920, 23667755,23893740, 22652285, 22872011, 14972841, 14274767];   
    
    for i in 0..x.len()-1
    {
        
        let skala_x = (max_x  - min_x ) / s;
        let mut x_trajektoria = ((x[i] - min_x ) / skala_x) as i32; 

        let skala_y = (max_y  - min_y ) / s;
        let mut y_trajektoria = ((y[i] - min_y ) / skala_y) as i32; 

        let skala_x = (max_x  - min_x ) / s;
        let mut x2_trajektoria = ((x[i+1] - min_x ) / skala_x) as i32; 

        let skala_y = (max_y  - min_y ) / s;
        let mut y2_trajektoria = ((y[i+1] - min_y ) / skala_y) as i32; 
        
        mapka.set_draw_color(Color::RGB(56 , 123 , 200));
        mapka.draw_line(( x_trajektoria+p_x, y_trajektoria+p_y ),(   x2_trajektoria+p_x , y2_trajektoria+p_y));
        mapka.set_draw_color(Color::RGB(100 , 143 , 100));
        
    }
    
    

  
    
  


}
fn punkty(x: i32 , y:i32 , rend: &mut WindowCanvas)
{
    rend.draw_point((x,y));
    rend.draw_point((x+1,y));
    rend.draw_point((x,y+1));
    rend.draw_point((x-1,y));
    rend.draw_point((x+1,y-1));
    rend.draw_point((x+1,y+1));
    rend.draw_point((x-1,y-1));
    rend.draw_point((x-1,y+1));
    rend.draw_point((x+1,y-1));
}






fn pause( imie: String) {
   let mut wejscie = String::new();
   println!( "Swietnie {}                    po wciśnieciu Enter nastąpi wypisanie wartosci których oczekujesz, bardzo się cieszę z naszej współpracy  ", imie );
    match io::stdin().read_line(&mut wejscie)  
    {
            Ok( _n) => 
            {
                //println!( "{n} bajtow do zapisu");
                println!("{wejscie}");

            }
            Err(_err) => println!(" niestety blad pobierania wartosci")

    }   
 
    
}

fn main() 
{
    
  
let mut skala: i64 = 600;
let ilosc_danych:usize ;
    let mut imie_uzytkownika = String::new();
    let mut plik_nazwa = String::new();
    let mut wartosc_nazwa = String::new();
    let mut offset_wartosc = 0.0;
    let mut offset_wartosc_s = String::new();
    let mut tak_nie = String::new();
    let current_dir = env::current_dir().expect("nie dziala");
    std::process::Command::new("clear").status().unwrap();
    println!("Aktualnie znajdujesz się: {}", current_dir.display().to_string());
    println!("********************************************************************************************************");
    println!("\n\t\t witaj w systemie satelitarnym, jak masz na imie?\n");
    println!("********************************************************************************************************");
    io::stdin().read_line(&mut imie_uzytkownika).expect("blad inputa");
    std::process::Command::new("clear").status().unwrap();
    println!("Witaj {}  teraz trzeba podac nazwę pliku do obróbki:" , imie_uzytkownika.trim());
    

    io::stdin().read_line(&mut plik_nazwa).expect("blad inputa");
    let path = Path::new(plik_nazwa.trim());
    println!("{}", current_dir.display().to_string()+"/"+&plik_nazwa.trim().to_string());
    let fileg = netcdf::open(path).expect("nie udalo sie otworzyc");
     
    println!("\n________________________Pole nazw wartosci_____________________\n");
    for i in fileg.variables()
    {
         println!(" nazwa:  {}     " , i.name() );   
    }   
    println!("\n_________dobra robota {} teraz trzeba wybrac dane które chcesz badać_______________\n", imie_uzytkownika);
    io::stdin().read_line(&mut wartosc_nazwa).expect("blad inputa");
    std::process::Command::new("clear").status().unwrap();
    let var2 = fileg.variable(wartosc_nazwa.trim()).expect("nie ma takiego pola  'var2'");
    println!("\n__________________Pole konfiguracji {} ___________________________\n", var2.name());
            for i in var2.attributes()
            {
               println!("{} = \t {:?}    " , i.name() , i.value().expect("nie dziala") );   
            }
            println!("____________dobrze {}\n                                sprawdz pole offset czy trzeba zmienic wartość jak nie wpisz 0.0__________________________", imie_uzytkownika.trim());
            io::stdin().read_line(&mut offset_wartosc_s).expect("blad inputa");
            if offset_wartosc_s != "0.0"
            {
                offset_wartosc = offset_wartosc_s.trim().parse().expect("zamiana sie nie udała");

            }
    println!( "___________________Ostatnia sprawa {} czy występuje pole coordinates = Str( lon_l1b_echo_plrm  lat_l1b_echo_plrm )\n  ------  t/n  ----- ", imie_uzytkownika );

            io::stdin().read_line(&mut tak_nie ).expect("blad inputa");

        
            pause(imie_uzytkownika);
let mut a: Vec<i64> = Vec::new();
let mut b: Vec<i64> = Vec::new();
let mut c: Vec<i64> = Vec::new();
    if tak_nie.trim() == "t"
    {
        let var3 = fileg.variable("lon_l1b_echo_plrm").expect("nie ma takiego pola  'var3'");
        let var4 = fileg.variable("lat_l1b_echo_plrm").expect("nie ma takiego pola  'var4'");
       
        let dataw = var2.values::<f64>(None, None).map(|x| (x) + offset_wartosc).expect("calosc");
       
        
        let szerokosc = var3.values::<f32>(None, None).map(|x| (x)).expect("calosc");
       
        let wysokosc = var4.values::<f32>(None, None).map(|x| (x)).expect("calosc");
       ilosc_danych = dataw.len();
    
        (0..dataw.len()).for_each(|i| {
            a.push(unsafe {dataw[i].to_int_unchecked()}) ;
            c.push(unsafe {szerokosc[i].to_int_unchecked()}) ;
            b.push(unsafe {wysokosc[i].to_int_unchecked()}) ;
            println!("| lp:{} | lon: {:?}  \t|  lat: {:?} \t| wartosc: {}  |",i+1, szerokosc[i]* 0.000001, wysokosc[i] * 0.000001, dataw[i] * 0.0001);
         
         
        }); 
    }else 
    {
        let dataw = var2.values::<f64>(None, None).map(|x| (x  * 0.0001) + offset_wartosc).expect("calosc");
        (0..dataw.len()).for_each(|i| {
            println!("| lp:{} \t| wartosc: {}  |",i+1, dataw[i]);
           
        }); 
          
    }
    
     


            

    //let var3 = fileg.variable("lon_l1b_echo_plrm").expect("nie ma takiego pola dupku 'var3'");
    //let var4 = fileg.variable("lat_l1b_echo_plrm").expect("nie ma takiego pola dupku 'var4'");
    //let var5 = fileg.attribute("Conventions").expect("nie ma takiego pola dupku 'var5'");
    //let var6 = fileg.dimension("echo_sample_ind").expect("nie ma takiego pola dupku 'var5'");
    //let var4= fileg.attribute("add_offset").expect("nie pobrano");
    //let data_i32 = var2.value::<i32>(Some(&[0, 9])).expect("nie odczytalem inta");
    //let data_f32 : f32 = var2.value(None).expect("nie odczytalem float");
    
    
    
    //let atrybut = var5.value();
    //let dim = var6.len(); 
        
   
    println!("\n__________________Atrybuty  ___________________________\n");   
    for i in fileg.attributes(){
        println!("{:?}                         \t {:?}" , i.name() , i.value().expect("moze ok"));
        }
        
            println!("\n__________________Globalne wymiary ___________________________\n");
            for i in fileg.dimensions(){
                
                println!(" {} \t----- {} " , i.name() , i.len() );
                }
               
                let sdl_context = sdl2::init().unwrap();
                let video_subsystem = sdl_context.video().unwrap();
            
                let window = video_subsystem.window("dzialajacy", 1200, 900)
                    .position_centered()
                
                    .build()
                    .unwrap();
            
                let mut canvas = window.into_canvas().build().unwrap();
                canvas.set_draw_color(Color::RGB(0,  0, 0));
                
                let mut event_pump = sdl_context.event_pump().unwrap();
               let mut x = 0;
               let mut y = 0;
///////////////////////////////////////////////////////////////
let mut max = a[0];             
let mut min = a[0];
let mut max_gx = b[0];             
let mut min_gx = b[0];
let mut max_gy = c[0];             
let mut min_gy = c[0];
(0..a.len()).for_each(|f| {
                
    if a[f] > max
    {
        max = a[f];
        
    }
    
    if a[f] < min
    {
        min = a[f];
    }
    
    if b[f] > max_gx
    {
        max_gx = b[f];
        
    }
    
    if b[f] < min_gx
    {
        min_gx = b[f];
    }

    if c[f] > max_gy
    {
        max_gy = c[f];
        
    }
    
    if c[f] < min_gy
    {
        min_gy = c[f];
    }

     print!("x:{}  y:{}  wartosc:{}  min:{}  max:{} \n",b[f] , c[f], a[f] , min , max );
    
   
    
});
/////////////////////////////////////////////////////////////////
                'running: loop {
                   
                   
            
                    for event in event_pump.poll_iter() {
                        match event 
                        {
                            Event::Quit {..} |
                            Event::KeyDown { keycode: Some(Keycode::Escape   ), .. } => {
                                break 'running
                                
                            },
                            Event::KeyDown { keycode: Some(Keycode::Left   ), .. } => 
                            {
                                x -=50;
                            },
                            Event::KeyDown { keycode: Some(Keycode::Right   ), .. } => 
                            {
                                x +=50;
                            },
                            Event::KeyDown { keycode: Some(Keycode::Up   ), .. } => 
                            {
                                y -=50;
                            },
                            Event::KeyDown { keycode: Some(Keycode::Down   ), .. } => 
                            {
                                y +=50;
                            },
                            Event::KeyDown { keycode: Some(Keycode::Q   ), .. } => 
                            {
                                skala -=200;
                            },Event::KeyDown { keycode: Some(Keycode::A   ), .. } => 
                            {
                                skala +=500;
                                y -=250;
                                x -=290;
                            },
            
                            _ => {}
                        }
                        punkty(50+x ,50+y , &mut canvas);
                    }
                    // The rest of the game loop goes here...
                   // tu wyswietlane sa punkty
                   (0..a.len()).for_each(|f| {
                
                    
                    //print!("{} {} {} \n",a[f] , b[f], c[f]);
                    
                    wyswietl( &mut canvas ,min_gx ,max_gx ,b[f].try_into().unwrap(), min_gy ,max_gy ,c[f].try_into().unwrap() , max , min , a[f].try_into().unwrap() , x, y,  skala);
                    mapa(&mut canvas, min_gx, max_gx, min_gy, max_gy, x , y,  skala);
                    
                });
                //print!("max: {} min: {}", max , min);  
                   
                
                  
                    
                 
                       
                  
          

                    
                    canvas.present();
                    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
                    canvas.clear();
                }            
            
}





