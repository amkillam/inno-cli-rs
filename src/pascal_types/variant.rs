//The Variant data type in Pascal provides a flexible general purpose data type. It can hold anything but structured data and pointers.
//As such, we use dynamic dispatch and a simple type alias to handle this.


// pub struct PascalVariant{
//     data: Box<dyn std::any::Any>
// }

pub type PascalVariant = dyn std::any::Any;