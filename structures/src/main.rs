struct Rectangle {
    width: u32,
    height:u32
}

fn main() {
    let width = 10; let height =12;
    let rect : Rectangle = get_rect(width,height);
    let area = get_area(rect);
    print(area);
}

fn get_rect(width:u32,height:u32) -> Rectangle{
    Rectangle {width,height}
}

fn get_area(rect : Rectangle)-> u32{
    rect.width * rect.height
}

fn print(area:u32){
    println!("THe ArEa is {}",area);
}
