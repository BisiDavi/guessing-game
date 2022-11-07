// sing Twelve days of Christmas

fn main(){
    let days: [&str; 12] = ["first","second","third","fourth","fifth","sixth","seventh","eighth","ninth","tenth","eleventh","twelfth"];
    for day in days.iter(){
        chorus(day);
    }
}

fn chorus(day:&str){
    println!("On the {} day of Christmas", day);
    println!("My good friends brought to me");
    println!("A song ad a christmas tree\n");
    format_day_chorus(day);
}

fn format_day_chorus(day:&str)-> &str{
    if day == "first" {
        return ""
    }else if day == "second"{
        return "Two candy canes"
    }else if day == "third"{
        return "Three boughs of holly\nTwo candy canes"
    }else if day == "fourth"{
        return "\nFour coloured lights\nThree boughs of holly\nTwo candy canes"
    }else if day == "fifth"{
        return "A shining star\nFour coloured lights\nThree boughs of holly\nTwo candy canes"
    }else if day == "sixth"{
        return "Little silver bells\nA shining star\nFour coloured lights\nThree boughs of holly\nTwo candy canes"
    }else if day == "seventh"{
        return "Candles a-glowing\nLittle silver bells\nA shining star\nFour coloured lights\nThree boughs of holly\nTwo candy canes"
    }else if day == "eighth"{
        return "Gold and silver tinsel\nCandles a-glowing\nLittle silver bells\nA shining star\nFour coloured lights\nThree boughs of holly\nTwo candy canes"
    }else if day == "ninth"{
        return "A guardian angel\nGold and silver tinsel\nCandles a-glowing\nLittle silver bells\nA shining star\nFour coloured lights\nThree boughs of holly\nTwo candy canes"
    }else if day == "tenth"{
        return "Some mistleo\nA guardian angel\nGold and silver tinsel\nCandles a-glowing\nLittle silver bells\nA shining star\nFour coloured lights\nThree boughs of holly\nTwo candy canes"
    }else if day == "eleventh"{
        return "Gifts for one and all\nSome mistleo\nA guardian angel\nGold and silver tinsel\nCandles a-glowing\nLittle silver bells\nA shining star\nFour coloured lights\nThree boughs of holly\nTwo candy canes"
    }else{
        return "All their good wishes\nGifts for one and all\nSome mistleo\nA guardian angel\nGold and silver tinsel\nCandles a-glowing\nLittle silver bells\nA shining star\nFour coloured lights\nThree boughs of holly\nTwo candy canes"
    }

}