use super::*;

use std::time::Instant;

struct Timer2{
    a:std::time::Instant
}

impl Timer2{
    pub fn new()->Timer2{
        Timer2{a:Instant::now()}
    }

    ///Returns the time since this object was created in seconds.
    pub fn elapsed(&self)->f64{
        let elapsed = self.a.elapsed();

        let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
        sec
    }
}


///Produces the total time each level took.
///Starts at the root and ends at the leaf level
pub struct TreeTimeResultIterator(
    std::vec::IntoIter<f64>
);
impl std::iter::FusedIterator for TreeTimeResultIterator{}
impl std::iter::ExactSizeIterator for TreeTimeResultIterator{}
unsafe impl std::iter::TrustedLen for TreeTimeResultIterator{}


impl Iterator for TreeTimeResultIterator{
    type Item=f64;
    fn next(&mut self)->Option<Self::Item>{
        self.0.next()
    }
    fn size_hint(&self)->(usize,Option<usize>){
        self.0.size_hint()
    }
}

///A generic timer trait is provided so that users can
///Opt out of the time measuring by providing a dummy implementation of this trait.
pub trait TreeTimerTrait:Sized+Send{
    type Bag:Send+Sized;
    ///Combine the timer results from the children.
    fn combine(a:Self::Bag,b:Self::Bag)->Self::Bag;
    ///Create a
    fn leaf_finish(self)->Self::Bag;

    ///Start the timer.
    fn start(&mut self);
    ///Stop the timer, record it, and create the timers for the children.
    fn next(self)->(Self,Self);
}

///The dummy version of the timer trait.
pub struct TreeTimerEmpty;
///The dummy returned time record.
pub struct BagEmpty;
impl TreeTimerTrait for TreeTimerEmpty{
    type Bag=BagEmpty;
    fn combine(_a:BagEmpty,_b:BagEmpty)->BagEmpty{
        BagEmpty
    }
   
    fn leaf_finish(self)->BagEmpty{
        BagEmpty
    }

    fn start(&mut self){
    }

    fn next(self)->(Self,Self){
        (TreeTimerEmpty,TreeTimerEmpty)
    }

}

///The returned time record at a particular level.
pub struct Bag{
    a:Vec<f64>
}
impl Bag{
    pub fn into_iter(self)->TreeTimeResultIterator{
        TreeTimeResultIterator(self.a.into_iter())
    }
}

///Used when the user wants the time to be returned.
pub struct TreeTimer2{
    a:Vec<f64>,
    index:usize,
    timer:Option<Timer2>
}

impl TreeTimer2{
    pub fn new(height:usize)->TreeTimer2{
        let v=(0..height).map(|_|0.0).collect();
        
        TreeTimer2{a:v,index:0,timer:None}
    }
}

impl TreeTimerTrait for TreeTimer2{
    type Bag=Bag;
    fn combine(mut a:Bag,b:Bag)->Bag{
        for (i,j) in a.a.iter_mut().zip(b.a.iter()){
            *i+=j;
        }
        a
    }
    

    //Can be called prematurely if there are no children
    fn leaf_finish(self)->Bag{

        let TreeTimer2{mut a,index,timer}=self;
        //debug_assert!(index==a.len()-1);
        a[index]+=timer.unwrap().elapsed();
        Bag{a:a}
    }

    fn start(&mut self){
        self.timer=Some(Timer2::new())
    }

    fn next(self)->(TreeTimer2,TreeTimer2){
        let TreeTimer2{mut a,index,timer}=self;
        a[index]+=timer.unwrap().elapsed();

        let b=(0..a.len()).map(|_|0.0).collect();
        (
            TreeTimer2{a:a,index:index+1,timer:None},
            TreeTimer2{a:b,index:index+1,timer:
                None}
        )
    }

  
}