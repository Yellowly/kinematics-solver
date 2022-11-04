use std::thread::current;

use yew::prelude::*;
use web_sys::{HtmlInputElement, HtmlCanvasElement, CanvasRenderingContext2d};
use wasm_bindgen::{JsCast, JsValue};


fn main() {
    println!("Hello, world!");
    yew::start_app::<MainComponent>();
}

enum Msg{
    Input(String, u32),
    Enter,
    None,
}

struct MainComponent{
    inputs: Vec<String>
}
impl Component for MainComponent{
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self{inputs: vec!["".to_string(); 5]}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool{
        match msg{
            Msg::Input(content, idx) => {
                self.inputs[idx as usize]=content;
                false
            }
            Msg::Enter => {
                true
            }
            Msg::None => {
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        //let test_vals: Vec<f64> = vec![1.0,2.0,2.0,2.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,3.0,4.0,4.0];
        let test_vals: Vec<f64> = vec![1.0,1.0,2.0,2.0,2.0,3.0,3.0,3.0,3.0,4.0,4.0,4.0,5.0,5.0,6.0];
        //background, secondary, text, accent, lines
        //#505050 or #808080 #ffffff for text
        let colors = vec!["#303030".to_string(), "#404040".to_string(), "#808080".to_string(), "#e2b831".to_string(), "#000000".to_string()];
        let link = ctx.link();
        html!{
            <>
                <div class="content-grid center-block">
                    <div class="input-div maintextcolor">
                        <p>{"Initial Velocity:"}</p>
                        <input class="numfield center-block" type="text" id="vi" name="vi" value={self.inputs[0].to_string()} oninput={link.callback(|event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); Msg::Input(input.value(),0)})}/>
                        <p>{"Final Velocity:"}</p>
                        <input class="numfield center-block" type="text" id="vf" name="vf" value={self.inputs[1].to_string()} oninput={link.callback(|event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); Msg::Input(input.value(),1)})}/>
                        <p>{"Acceleration:"}</p>
                        <input class="numfield center-block" type="text" id="a" name="a" value={self.inputs[2].to_string()} oninput={link.callback(|event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); Msg::Input(input.value(),2)})}/>
                        <p>{"Time:"}</p>
                        <input class="numfield center-block" type="text" id="t" name="t" value={self.inputs[3].to_string()} oninput={link.callback(|event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); Msg::Input(input.value(),3)})}/>
                        <p>{"Displacement:"}</p>
                        <input class="numfield center-block" type="text" id="dx" name="dx" value={self.inputs[4].to_string()} oninput={link.callback(|event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); Msg::Input(input.value(),4)})}/>
                        <button class="center-block" onclick={link.callback(|_| Msg::Enter)}>{"Solve"}</button>
                    </div>
                    <div class="solution-div">
                        <p>{"yo"}</p>
                    </div>
                </div>
            </>
        }
    }
}

#[derive(Clone)]
struct KinemEquatSolvr{
    vi: f64,
    vf: f64,
    acc: f64,
    time: f64,
    dx: f64,
    secans: Option<Vec<f64>>,
    work: Vec<String>,
}
impl KinemEquatSolvr{
    fn fromarr(vals: &[String; 5]) -> Result<KinemEquatSolvr, String>{
        let mut unknowns: u8 = 0;
        for v in vals{
            if v==""{unknowns+=1};
        }
        if unknowns!=2{
            return Result::Err("Must have exactally 2 unknown values to solve!".to_string())
        }
        let mut temp: KinemEquatSolvr = Self{vi: vals[0].parse::<f64>().unwrap_or(f64::NAN), vf: vals[1].parse::<f64>().unwrap_or(f64::NAN), acc: vals[2].parse::<f64>().unwrap_or(f64::NAN), time: vals[3].parse::<f64>().unwrap_or(f64::NAN), dx: vals[4].parse::<f64>().unwrap_or(f64::NAN), work:Vec::new(), secans:Option::None};
        for i in 0..vals.len(){
            if &vals[i]==""{
                match i{
                    0 => temp.solvevi(),
                    1 => temp.solvevf(),
                    2 => temp.solveacc(),
                    3 => temp.solvetime(),
                    _ => temp.solvedx()
                }
            }
        }
        Result::Ok(temp)
    }
    fn solveacc(&mut self){
        if self.vi==f64::NAN {
            self.work.push("Substitute values into Δd=v₁t-0.5at² :".to_string());
            self.work.push(format!("{}={}*{}-0.5*a*{}*{}",self.dx,self.vf,self.time,self.time,self.time));
            self.work.push("Solve for a :".to_string());
            self.work.push(format!("a=({}-{}*{})/(-0.5*{}*{})",self.dx,self.vf,self.time,self.time,self.time));
            self.acc=(self.dx-self.vf*self.time)/(-0.5*self.time*self.time);
        }else if self.vf==f64::NAN {
            self.work.push("Substitute values into Δd=v₀t+0.5at² :".to_string());
            self.work.push(format!("{}={}*{}+0.5*a*{}*{}",self.dx,self.vi,self.time,self.time,self.time));
            self.work.push("Solve for a :".to_string());
            self.work.push(format!("a=({}-{}*{})/(0.5*{}*{})",self.dx,self.vi,self.time,self.time,self.time));
            self.acc=(self.dx-self.vi*self.time)/(0.5*self.time*self.time);
        }else if self.time==f64::NAN {
            self.work.push("Substitute values into v₁²=v₀²+2aΔd :".to_string());
            self.work.push(format!("{}*{}={}*{}+2*a*{}",self.vf,self.vf,self.vi,self.vi,self.dx));
            self.work.push("Solve for a :".to_string());
            self.work.push(format!("a=({}*{}-{}*{})/(2*{})",self.vf,self.vf,self.vi,self.vi,self.dx));
            self.acc=(self.vf*self.vf-self.vi*self.vi)/(2.0*self.dx);
        }else {
            self.acc=(self.vf-self.vi)/self.time;
        }
    }
    fn solvedx(&mut self){
        if self.vi==f64::NAN{
            self.dx=self.vf*self.time-0.5*self.acc*self.time*self.time;
        }else if self.vf==f64::NAN{
            self.dx=self.vi*self.time+0.5*self.acc*self.time*self.time;
        }else{
            self.dx=(self.vf*self.vf-self.vi*self.vi)/(2.0*self.acc);
        }
    }
    fn solvetime(&mut self){
        if self.vi==f64::NAN{
            self.time=(-self.vf+(self.vf*self.vf+2.0*self.acc*self.dx).sqrt())/self.acc;
            self.secans=Option::Some(vec![(-self.vf-(self.vf*self.vf+2.0*self.acc*self.dx).sqrt())/self.acc]);
        }else if self.vf==f64::NAN{
            self.time=(-self.vi+(self.vi*self.vi+2.0*self.acc*self.dx).sqrt())/self.acc;
            self.secans=Option::Some(vec![(-self.vi-(self.vi*self.vi+2.0*self.acc*self.dx).sqrt())/self.acc]);
        }else{
            self.time=(self.vf-self.vi)/self.acc;
        }
    }
    fn solvevi(&mut self){
        if self.vf==f64::NAN{
            self.vi=(0.5*self.acc*self.time*self.time-self.dx)/self.time;
        }else{
            self.vi=self.vf-self.acc*self.time;
            if let Option::Some(ref mut vector)=self.secans{
                vector.push(self.vf-self.acc*vector[0]);
            }
        }
    }
    fn solvevf(&mut self){
        self.vf=self.vi+self.acc*self.time;
        if let Option::Some(ref mut vector)=self.secans{
            vector.push(self.vi+self.acc*vector[0]);
        }
    }

}

