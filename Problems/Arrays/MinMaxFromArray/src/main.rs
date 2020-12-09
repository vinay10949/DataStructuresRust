//Find maximum and minimum nos in array

//SC O(1)

//T(N/2)


//#![feature(core_intrinsics)]
#[warn(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}



#[warn(non_snake_case)]
fn main() {
    let cin = std::io::stdin();
    let mut s = String::new();
    cin.read_line(&mut s).unwrap();

    let values = s
        .split_whitespace()
        .map(|x| x.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()
        .unwrap();


    let (min,max)=find_max(values);
    println!(" Min = {} , Max = {} ",min,max);
}


fn find_max(values :Vec<i32>)->(i32,i32){
    println!("No of values in vector {:?}", values.len());
    let (mut min,mut max)=(0,0);
    if values.len()==0{
        assert!(values.len()>0);

    }else if values.len()==1{
        return (values[0],values[0]);

    }else if values.len()==2{
        if values[0]<values[1]{
            return (values[0],values[1]);
        }else{
            return (values[1],values[0]);
        }

    }else{
        if values[0]<values[1]{
            min=values[0];
            max=values[1];
        }else{
            min=values[1];
            max=values[0];

        }
            for i in (0..values.len()).step_by(2){
                if i<=values.len()-2{

          //          println!("{} {} {} ",i,values[i],values[i+1]);

                    if values[i]<values[i+1]{
                        if min>values[i]{
                            min=values[i];
                        }
                        if max<values[i+1]{
                            max=values[i+1];
                        }

                    }else{
                        if min>values[i+1]{
                            min=values[i+1];
                        }
                        if max<values[i]{
                            max=values[i];
                        }

                    }

                }else{
                    println!("{} {} {} ",min,max,values[i]);

                        if min>values[i]{
                            min=values[i];
                        }
                        if max<values[i]{
                            max=values[i];
                        }


                }
           //     println!("Min {} Max {} ",min,max)


            }

    }

    return (min,max);
}
