use std::io; // standart lib. input/output
use std::cmp::Ordering; // standart lib. compare(karşılaştırma) ve ordering 
use rand::Rng; // random number generator

fn main(){
    println!("Sayıyı Tahmin edin!!");
    let secret_number = rand::thread_rng().gen_range(1..=100); // tahmin edileceksayıyı rng ile oluşturduk 1 den 100 e 100 dahil

    loop { // bulana kadar döngü
    println!("Tahmininizi giriniz : ");
    let mut guess = String::new(); //user input için string tanımlaması
    io::stdin().read_line(& mut guess).expect("Failed to read line"); // girilen değeri guess değişkenine atıyor

    let guess_sayi:u32 = match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=>continue,
    };  // girileni parse edip u32 sayı olan guess_sayi değişkenine atıyor 

    //println!("Tahmininiz:{guess}");
    match guess_sayi.cmp(&secret_number) {
        Ordering::Less=> println!("yükselt"),
        Ordering::Greater=> println!("alçalt"),
        Ordering::Equal=> {println!("oleyyy bildin!!");
        //println!("gizli sayı  : {secret_number}");
        break;}
    }  // girilen sayı ile random sayıyı karşılaştırıp buna göre mesaj basıyor.  bilince loop "break" ile duruyor

    }
    
}

