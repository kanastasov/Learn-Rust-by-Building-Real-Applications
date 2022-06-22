use std::str::Utf8Error;
use super::method::{MethodError, Method};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Result as FmtResult;
use std::fmt::Formattter;
use std::fmt::Debug;      
use std::str;
use super::QueryString;

pub struct Request<'buf> {
     path: &'buf str,
     query_string:Option<QueryString<'buf>>,
     method: Method,

 } 

 impl Request {
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {}
 }

 impl<'buf> TryFrom<& 'buf [u8]> for Request<'buf>{
    type Error = ParseError;


    fn try_from<'a>(buf &'a [u8]) -> Result <Self, Self:Error>{
        let string = String::from("asd");
        string.encrypt();
        buf.encrypt();

        match str: from_utf8(buf){
         Ok(request) => {

         },
         Err(_) => return Err(ParseError::InvalidEncoding),

        }

        match str: from_utf8(buf).or(Err(ParseError::InvalidEncoding)){
         Ok(request) => {

         },
         Err(e) => return Err(e),

        }

        let request = str::from_utf8(buf)?;

      //   match get_next_word(request) [
      //    Some((method, request)) => {},
      //    None => return Err(ParseError::InvalidRequest),
      //   ]

        let (method, request ) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request ) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol,_) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        if protocol != "HTTP/1.1"{
            return Err(ParseError::InvalidProtocol);
        } 

        let method: Method = method.parse()?;

        let mut query_string = None;

      //   match path.find("?"){
      //    Some(i) => {
      //       query_string = Some(&path[i..]);
      //       path = &path[..i];
      //    }
      //    None => {}
      //   }

      //   let q = path.find('?');
      //   if q.is_some() {
      //    let i = q.unwrap();
      //    query_string = Some(&path[i..]);
      //    path = &path[..i];
      //   }


        if let Some(i) = path.find('?'){
         query_string = Some(QueryString::from(&path[i..]));
         path = &path[..i];
        }

        Ok(Self {
         path: path,
         query_string, 
         method,

        })
    }

 }

 fn get_next_word(request: & str, b : &'b str) -> Option<(&'a str, &'b str)> {
   for(i, c) in request.chars().enumerate(){
      if c ==  ' '{
         return Some((&request[..i], &request[i+1..]))
      }
   }
   None
 }

 impl Display for ParseError{
   fn fmt(&slef, f: &mut Formatter) -> FmtResult{}
   write!(f, "{}", self.message())
 }

 impl Debug for ParseError{
   fn fmt(&slef, f: &mut Formatter) -> FmtResult{}
   write!(f, "{}", self.message())
 }

pub enum ParseError{
   InvalidRequest,
   InvalidEncoding,
   InvalidProtocol,
   InvalidMethod,

}
impl ParseError{
   fn message(&self) -> &str{
      match self {
        Self::InvalidRequest =>  "Invalid Request",
        Self::InvalidEncoding => "Invalid Encoding",
        Self::InvalidProtocol => "Invalid Protocol",
        Self::InvalidMethod =>  "Invalid Method",
      }
   }
}

impl From<MethodError> for ParseError{
   fn from (_: MethodError) -> Self {
      Self::InvalidMethod
   }
}


impl From<Utf8Error> for ParseError{
   fn from (_: Utf8Error) -> Self {
      Self::InvalidEncoding
   }
}


impl Error for ParseError{}


//  trait Encrypt {
//     fn encrypt (&self) -> Self;
//  }

//  impl Encrypt for String {
//     fn encrypt(&self) -> Self {
//         unimplemented!()
//     }
//  }

//  impl Encrypt for &[u8]{
//     fn encrypt(&self) -> Self {
//         unimplemented!()
//     }
//  }