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
    inputs: Vec<String>,
    work: Vec<String>
}
impl Component for MainComponent{
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self{inputs: vec!["".to_string(); 5], work: Vec::default()}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool{
        match msg{
            Msg::Input(content, idx) => {
                self.inputs[idx as usize]=content;
                false
            }
            Msg::Enter => {
                self.work=Vec::new();
                let res: Result<KinemEquatSolvr,String> = KinemEquatSolvr::fromarr(&self.inputs);
                match res{
                    Ok(ref r) => {self.inputs=r.to_str_arr(); self.work.append(&mut r.work.clone());},
                    Err(ref s) => self.work.push(s.clone())
                }
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
                <div class="title center-block">
                    <h1 class="center-block maintextcolor">{"Budget Kinematics Solver"}</h1>
                    <hr/>
                </div>
                <div class="content-grid center-block">
                    <div class="input-div maintextcolor">
                        <div class="max-space-content bgcol1">
                        <p>{"Initial Velocity:"}</p>
                        <input class="center-block" type="text" id="vi" name="vi" value={self.inputs[0].to_string()} oninput={link.callback(|event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); Msg::Input(input.value(),0)})}/>
                        <p>{"Final Velocity:"}</p>
                        <input class="center-block" type="text" id="vf" name="vf" value={self.inputs[1].to_string()} oninput={link.callback(|event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); Msg::Input(input.value(),1)})}/>
                        <p>{"Acceleration:"}</p>
                        <input class="center-block" type="text" id="a" name="a" value={self.inputs[2].to_string()} oninput={link.callback(|event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); Msg::Input(input.value(),2)})}/>
                        <p>{"Time:"}</p>
                        <input class="center-block" type="text" id="t" name="t" value={self.inputs[3].to_string()} oninput={link.callback(|event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); Msg::Input(input.value(),3)})}/>
                        <p>{"Displacement:"}</p>
                        <input class="center-block" type="text" id="dx" name="dx" value={self.inputs[4].to_string()} oninput={link.callback(|event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); Msg::Input(input.value(),4)})}/>
                        <button class="center-block" onclick={link.callback(|_| Msg::Enter)}>{"Solve"}</button>
                        </div>
                    </div>
                    <div class="solution-div maintextcolor">
                        <div class="max-space-content bgcol1">
                        {self.work.iter().enumerate().map(|(i, s)| {
                            if s.starts_with("&"){
                                if s.starts_with("&2"){html!{<p class="sectextcolor">{&s[2..]}</p>}}
                                else if s.starts_with("&3"){html!{<p class="acctextcolor">{&s[2..]}</p>}}
                                else {html!{<p class="maintextcolor">{&s[2..]}</p>}}
                            }else{
                                html!{<p>{s.clone()}</p>}
                            }
                        }).collect::<Html>()}
                        </div>
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
    fn fromarr(vals: &[String]) -> Result<KinemEquatSolvr, String>{
        let mut unknowns: u8 = 0;
        for v in vals{
            if v==""{unknowns+=1};
        }
        if unknowns!=2{
            if unknowns==0 { return Result::Err("All values are already known! Nothing to solve.".to_string())}
            return Result::Err("Must have exactally 2 unknown values to solve!".to_string())
        }
        let mut temp: KinemEquatSolvr = Self{vi: vals[0].parse::<f64>().unwrap_or(f64::NAN), vf: vals[1].parse::<f64>().unwrap_or(f64::NAN), acc: vals[2].parse::<f64>().unwrap_or(f64::NAN), time: vals[3].parse::<f64>().unwrap_or(f64::NAN), dx: vals[4].parse::<f64>().unwrap_or(f64::NAN), work:Vec::new(), secans:Option::None};
        for i in 0..vals.len(){
            if &vals[KinemEquatSolvr::convert_idx_order(i)]==""{
                match i{
                    0 => temp.solveacc(),
                    1 => temp.solvedx(),
                    2 => temp.solvetime(),
                    3 => temp.solvevi(),
                    _ => temp.solvevf()
                }
            }else if temp.get_as_idx(Self::convert_idx_order(i))==f64::NAN{
                return Result::Err(format!("Error parsing input in input field #{}",i));
            }
        }
        Result::Ok(temp)
    }
    fn get_as_idx(&self, idx: usize) -> f64{
        let tempsecans: Vec<f64> = match &self.secans{
            Some(v) => v.clone(),
            None => vec![f64::NAN; 3]
        };
        return match idx{
            0 => self.vi,
            1 => self.vf,
            2 => self.acc, 
            3 => self.time,
            4 => self.dx,
            5 => tempsecans[0],
            6 => tempsecans[1],
            7 => tempsecans[2],
            _ => f64::NAN,
        }
    }
    fn solveacc(&mut self){
        if self.vi.is_nan() {
            self.work.push("&2Substitute values into Δd=v₁t-0.5at² :".to_string());
            self.work.push(format!("{}={}*{}-0.5*a*{}*{}",self.dx,self.vf,self.time,self.time,self.time));
            self.work.push("&2Solve for a :".to_string());
            self.work.push(format!("a=({}-{}*{})/(-0.5*{}*{})",self.dx,self.vf,self.time,self.time,self.time));
            self.acc=(self.dx-self.vf*self.time)/(-0.5*self.time*self.time);
        }else if self.vf.is_nan() {
            self.work.push("&2Substitute values into Δd=v₀t+0.5at² :".to_string());
            self.work.push(format!("{}={}*{}+0.5*a*{}*{}",self.dx,self.vi,self.time,self.time,self.time));
            self.work.push("&2Solve for a :".to_string());
            self.work.push(format!("a=({}-{}*{})/(0.5*{}*{})",self.dx,self.vi,self.time,self.time,self.time));
            self.acc=(self.dx-self.vi*self.time)/(0.5*self.time*self.time);
        }else if self.time.is_nan() {
            self.work.push("&2Substitute values into v₁²=v₀²+2aΔd :".to_string());
            self.work.push(format!("{}*{}={}*{}+2*a*{}",self.vf,self.vf,self.vi,self.vi,self.dx));
            self.work.push("&2Solve for a :".to_string());
            self.work.push(format!("a=({}*{}-{}*{})/(2*{})",self.vf,self.vf,self.vi,self.vi,self.dx));
            self.acc=(self.vf*self.vf-self.vi*self.vi)/(2.0*self.dx);
        }else{
            self.work.push("&2Substitute values into v₁=v₀+at :".to_string());
            self.work.push(format!("{}={}+a*{}",self.vf,self.vi,self.time));
            self.work.push("&2Solve for a :".to_string());
            self.work.push(format!("a=({}-{})/{}",self.vf,self.vi,self.time));
            self.acc=(self.vf-self.vi)/self.time;
        }
        self.work.push(format!("&3a={}",self.acc));
    }
    fn solvedx(&mut self){
        if self.vi.is_nan(){
            self.work.push("&2Substitute values into Δd=v₁t-0.5at² :".to_string());
            self.work.push(format!("Δx={}*{}-0.5*{}*{}*{}",self.vf,self.time,self.acc,self.time,self.time));
            self.dx=self.vf*self.time-0.5*self.acc*self.time*self.time;
        }else if self.vf.is_nan(){
            self.work.push("&2Substitute values into Δd=v₀t+0.5at² :".to_string());
            self.work.push(format!("Δx={}*{}+0.5*{}*{}*{}",self.vf,self.time,self.acc,self.time,self.time));
            self.dx=self.vi*self.time+0.5*self.acc*self.time*self.time;
        }else{
            self.work.push("&2Substitute values into v₁²=v₀²+2aΔd :".to_string());
            self.work.push(format!("{}*{}={}*{}+2*{}*Δd",self.vf,self.vf,self.vi,self.vi,self.acc));
            self.work.push("&2Solve for Δx :".to_string());
            self.work.push(format!("Δx=({}*{}-{}*{})/(2*{})",self.vf,self.vf,self.vi,self.vi,self.acc));
            self.dx=(self.vf*self.vf-self.vi*self.vi)/(2.0*self.acc);
        }
        self.work.push(format!("&3Δx={}",self.dx));
    }
    fn solvetime(&mut self){
        if self.vi.is_nan(){
            self.work.push("&2Substitute values into Δd=v₁t-0.5at² :".to_string());
            self.work.push(format!("{}={}*t-0.5*{}*t*t",self.dx,self.vf,self.acc));
            self.work.push("&2Solve for t using quadratic formula :".to_string());
            self.work.push(format!("0={}t²-{}t+{}",self.acc*0.5,self.vf,self.dx));
            self.work.push(format!("t=({}+-√({}*{}-4*{}*{}))/(2*{})",self.vf,self.vf,self.vf,self.acc*0.5,self.dx,self.acc*0.5));
            self.time=(self.vf+(self.vf*self.vf-2.0*self.acc*self.dx).sqrt())/self.acc;
            let t2: f64 = (self.vf-(self.vf*self.vf-2.0*self.acc*self.dx).sqrt())/self.acc;
            self.secans=Option::Some(vec![t2.clone()]);
            self.work.push(format!("&3t={}, {}",self.time, t2));
        }else if self.vf.is_nan(){
            self.work.push("&2Substitute values into Δd=v₀t+0.5at² :".to_string());
            self.work.push(format!("{}={}*t+0.5*{}*t*t",self.dx,self.vi,self.acc));
            self.work.push("&2Solve for t using quadratic formula :".to_string());
            self.work.push(format!("0={}t²+{}t-{}",self.acc*0.5,self.vi,self.dx));
            self.work.push(format!("t=(-{}+-√({}*{}+4*{}*{}))/(2*{})",self.vi,self.vi,self.vi,self.acc*0.5,self.dx,self.acc*0.5));
            self.time=(-self.vi+(self.vi*self.vi+2.0*self.acc*self.dx).sqrt())/self.acc;
            let t2: f64 = (-self.vi-(self.vi*self.vi+2.0*self.acc*self.dx).sqrt())/self.acc;
            self.secans=Option::Some(vec![t2.clone()]);
            self.work.push(format!("&3t={}, {}",self.time, t2));
        }else{
            self.work.push("&2Substitute values into v₁=v₀+at :".to_string());
            self.work.push(format!("{}={}+{}*t",self.vf,self.vi,self.acc));
            self.work.push("&2Solve for t :".to_string());
            self.work.push(format!("t=({}-{})/{}",self.vf,self.vi,self.acc));
            self.time=(self.vf-self.vi)/self.acc;
            self.work.push(format!("&3t={}",self.time));
        }
    }
    fn solvevi(&mut self){
        if self.vf.is_nan(){
            self.work.push("&2Substitute values into Δd=v₀t+0.5at² :".to_string());
            self.work.push(format!("{}=v₀*{}+0.5*{}*{}*{}",self.dx,self.time,self.acc,self.time,self.time));
            self.work.push("&2Solve for v₀ :".to_string());
            self.work.push(format!("v₀=(0.5*{}*{}*{}-{})/{}",self.acc,self.time,self.time,self.dx,self.time));
            self.vi=(0.5*self.acc*self.time*self.time-self.dx)/self.time;
            self.work.push(format!("&3v₀={}",self.vi));
        }else{
            self.vi=self.vf-self.acc*self.time;
            if let Option::Some(ref mut vector)=self.secans{
                let vi2: f64 = self.vf-self.acc*vector[0];
                vector.push(vi2);
                self.work.push(format!("&3v₀={}, {}",self.vi, vi2));
            }else{
                self.work.push(format!("&3v₀={}",self.vi));
            }
        }
    }
    fn solvevf(&mut self){
        self.work.push("&2Substitute values into v₁=v₀+at :".to_string());
        self.work.push(format!("v₁={}+{}*{}",self.vi,self.acc,self.time));
        self.vf=self.vi+self.acc*self.time;
        if let Option::Some(ref mut vector)=self.secans{
            vector.push(f64::NAN);
            let vf2 = self.vi+self.acc*vector[0];
            vector.push(vf2);
            self.work.push(format!("&3v₀={}, {}",self.vf, vf2));
        }else{
            self.work.push(format!("&3v₁={}",self.vf));
        }
    }
    fn to_str_arr(&self) -> Vec<String>{
        let mut timestr: String = self.time.to_string();
        let mut vistr: String = self.vi.to_string();
        let mut vfstr: String = self.vf.to_string();
        if let Option::Some(ref vector)=self.secans{
            timestr.push_str(&format!(", {}",&vector[0].to_string()));
            if vector.len()==3{
                vfstr.push_str(&format!(", {}",&vector[2].to_string()));
            }else{
                vistr.push_str(&format!(", {}",&vector[1].to_string()));
            }
        }
        return vec![vistr, vfstr, self.acc.to_string(), timestr,self.dx.to_string()]
    }
    fn convert_idx_order(from: usize) -> usize{
        return match from{
            0 => 2,
            1 => 4,
            2 => 3,
            3 => 0,
            _ => 1
        }
    }
}

