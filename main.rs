//use nannou::draw::properties::SetOrientation;
use nannou::prelude::*;
//use nannou::time::DurationF64;



#[derive(Copy, Clone)]
struct Plot {
    x: f32,
    y: f32,
}


struct Model {
    land: Vec<Plot>,
    window_id: window::Id,
}


fn init(app: &App) -> Model {
    //window builder to call for resources to create a window.
    let window_id = app
    .new_window()
    .view(view)                        
    .build()
    .unwrap(); //crashes if error with creating window.

    //initialize a Vector of Plots. Each index will create a new plot on the canvas.
    let land = vec![
        //stars!
        Plot {x: -500.0, y: 200.0},
        Plot {x: -500.0, y: 210.0},
        Plot {x: -500.0, y: 220.0},
        Plot {x: -500.0, y: 230.0},
        Plot {x: -500.0, y: 240.0},
        Plot {x: -500.0, y: 250.0},
        Plot {x: -500.0, y: 260.0},
        //


        // 0 beginning!
        Plot { x: -500.0, y: -100.0},
        Plot { x: -500.0, y: -95.0},
        Plot { x: -500.0, y: -90.0},
        Plot { x: -500.0, y: -85.0},
        Plot { x: -500.0, y: -80.0},
        Plot { x: -500.0, y: -75.0},
        Plot { x: -500.0, y: -70.0},
        Plot { x: -500.0, y: -65.0},
        Plot { x: -500.0, y: -60.0},
        Plot { x: -500.0, y: -55.0},
        Plot { x: -500.0, y: -50.0},
        Plot { x: -500.0, y: -45.0},
        Plot { x: -500.0, y: -40.0},
        Plot { x: -500.0, y: -35.0},
        Plot { x: -500.0, y: -30.0},
        Plot { x: -500.0, y: -25.0},
        Plot { x: -500.0, y: -20.0},
        Plot { x: -500.0, y: -15.0},
        Plot { x: -500.0, y: -10.0},
        Plot { x: -500.0, y: -5.0},
        Plot { x: -500.0, y: 0.0},
        Plot { x: -495.0, y: 4.0},
        Plot { x: -488.0, y: 4.0},
        Plot { x: -481.0, y: 4.0},
        Plot { x: -477.0, y: 4.0},
        Plot { x: -470.0, y: 4.0},
        Plot { x: -463.0, y: 4.0},
        Plot { x: -456.0, y: 4.0},
        Plot { x: -449.0, y: 4.0},
        Plot { x: -442.0, y: 4.0},
        Plot { x: -435.0, y: 4.0},
        Plot { x: -430.0, y: 3.0},
        Plot { x: -430.0, y: 2.0},
        Plot { x: -495.0, y: -100.0},
        Plot { x: -488.0, y: -100.0},
        Plot { x: -481.0, y: -100.0},
        Plot { x: -477.0, y: -100.0},
        Plot { x: -470.0, y: -100.0},
        Plot { x: -463.0, y: -100.0},
        Plot { x: -456.0, y: -100.0},
        Plot { x: -449.0, y: -100.0},
        Plot { x: -442.0, y: -100.0},
        Plot { x: -435.0, y: -100.0},
        Plot { x: -430.0, y: -100.0},
        Plot { x: -433.0, y: -100.0},
        Plot { x: -430.0, y: -95.0},
        Plot { x: -430.0, y: -90.0},
        Plot { x: -430.0, y: -85.0},
        Plot { x: -430.0, y: -80.0},
        Plot { x: -430.0, y: -75.0},
        Plot { x: -430.0, y: -70.0},
        Plot { x: -430.0, y: -65.0},
        Plot { x: -430.0, y: -60.0},
        Plot { x: -430.0, y: -55.0},
        Plot { x: -430.0, y: -50.0},
        Plot { x: -430.0, y: -45.0},
        Plot { x: -430.0, y: -40.0},
        Plot { x: -430.0, y: -35.0},
        Plot { x: -430.0, y: -30.0},
        Plot { x: -430.0, y: -25.0},
        Plot { x: -430.0, y: -20.0},
        Plot { x: -430.0, y: -15.0},
        Plot { x: -430.0, y: -10.0},
        Plot { x: -430.0, y: -5.0},
        Plot { x: -430.0, y: 0.0},
        //0 ending!

        // / of x
        Plot { x: -400.0, y: -100.0},
        Plot { x: -395.0, y: -95.0},
        Plot { x: -390.0, y: -90.0},
        Plot { x: -385.0, y: -85.0},
        Plot { x: -380.0, y: -80.0},
        Plot { x: -375.0, y: -75.0},
        Plot { x: -370.0, y: -70.0},
        Plot { x: -365.0, y: -65.0},
        Plot { x: -360.0, y: -60.0},
        Plot { x: -355.0, y: -55.0},
        Plot { x: -350.0, y: -50.0},
        Plot { x: -345.0, y: -45.0},
        Plot { x: -340.0, y: -40.0},
        Plot { x: -335.0, y: -35.0},
        Plot { x: -330.0, y: -30.0},
        Plot { x: -325.0, y: -25.0},
        Plot { x: -320.0, y: -20.0},
        Plot { x: -315.0, y: -15.0},
        Plot { x: -310.0, y: -10.0},
        Plot { x: -305.0, y: -5.0},
        Plot { x: -300.0, y: 0.0},

        // \ of x
        Plot { x: -400.0, y: 0.0},
        Plot { x: -395.0, y: -5.0},
        Plot { x: -390.0, y: -10.0},
        Plot { x: -385.0, y: -15.0},
        Plot { x: -380.0, y: -20.0},
        Plot { x: -375.0, y: -25.0},
        Plot { x: -370.0, y: -30.0},
        Plot { x: -365.0, y: -35.0},
        Plot { x: -360.0, y: -40.0},
        Plot { x: -355.0, y: -45.0},
        Plot { x: -350.0, y: -50.0},
        Plot { x: -345.0, y: -55.0},
        Plot { x: -340.0, y: -60.0},
        Plot { x: -335.0, y: -65.0},
        Plot { x: -330.0, y: -70.0},
        Plot { x: -325.0, y: -75.0},
        Plot { x: -320.0, y: -80.0},
        Plot { x: -315.0, y: -85.0},
        Plot { x: -310.0, y: -90.0},
        Plot { x: -305.0, y: -95.0},
        Plot { x: -300.0, y: -100.0},

        //start of 1
        Plot { x: -250.0, y: 0.0},
        Plot { x: -245.0, y: 5.0},
        Plot { x: -240.0, y: 10.0},
        Plot { x: -235.0, y: 15.0},
        Plot { x: -230.0, y: 20.0},
        Plot { x: -230.0, y: 15.0},
        Plot { x: -230.0, y: 10.0},
        Plot { x: -230.0, y: 5.0},
        Plot { x: -230.0, y: 0.0},
        Plot { x: -230.0, y: -5.0},
        Plot { x: -230.0, y: -10.0},
        Plot { x: -230.0, y: -15.0},
        Plot { x: -230.0, y: -20.0},
        Plot { x: -230.0, y: -25.0},
        Plot { x: -230.0, y: -30.0},
        Plot { x: -230.0, y: -35.0},
        Plot { x: -230.0, y: -40.0},
        Plot { x: -230.0, y: -45.0},
        Plot { x: -230.0, y: -50.0},
        Plot { x: -230.0, y: -55.0},
        Plot { x: -230.0, y: -60.0},
        Plot { x: -230.0, y: -70.0},
        Plot { x: -230.0, y: -75.0},
        Plot { x: -230.0, y: -80.0},
        Plot { x: -230.0, y: -85.0},
        Plot { x: -230.0, y: -90.0},
        Plot { x: -230.0, y: -95.0},
        Plot { x: -230.0, y: -100.0},
        Plot { x: -260.0, y: -100.0},
        Plot { x: -255.0, y: -100.0},
        Plot { x: -250.0, y: -100.0},
        Plot { x: -245.0, y: -100.0},
        Plot { x: -240.0, y: -100.0},
        Plot { x: -235.0, y: -100.0},
        Plot { x: -230.0, y: -100.0},
        Plot { x: -225.0, y: -100.0},
        Plot { x: -220.0, y: -100.0},
        Plot { x: -215.0, y: -100.0},
        Plot { x: -210.0, y: -100.0},
        Plot { x: -205.0, y: -100.0},
        Plot { x: -200.0, y: -100.0},
        //end of 1


        //beginning of b
        Plot { x: -170.0, y: -100.0},
        Plot { x: -170.0, y: -95.0},
        Plot { x: -170.0, y: -90.0},
        Plot { x: -170.0, y: -85.0},
        Plot { x: -170.0, y: -80.0},
        Plot { x: -170.0, y: -75.0},
        Plot { x: -170.0, y: -70.0},
        Plot { x: -170.0, y: -65.0},
        Plot { x: -170.0, y: -60.0},
        Plot { x: -170.0, y: -55.0},
        Plot { x: -170.0, y: -50.0},
        Plot { x: -170.0, y: -45.0},
        Plot { x: -170.0, y: -40.0},
        Plot { x: -170.0, y: -35.0},
        Plot { x: -170.0, y: -30.0},
        Plot { x: -170.0, y: -25.0},
        Plot { x: -170.0, y: -20.0},
        Plot { x: -170.0, y: -15.0},
        Plot { x: -170.0, y: -10.0},
        Plot { x: -170.0, y: -5.0},
        Plot { x: -170.0, y: 0.0},
        Plot { x: -170.0, y: 5.0},
        Plot { x: -170.0, y: 10.0},
        Plot { x: -170.0, y: 15.0},
        Plot { x: -170.0, y: 20.0},

        Plot { x: -165.0, y: -50.0},
        Plot { x: -160.0, y: -50.0},
        Plot { x: -155.0, y: -50.0},
        Plot { x: -150.0, y: -50.0},
        Plot { x: -145.0, y: -50.0},
        Plot { x: -140.0, y: -50.0},
        Plot { x: -135.0, y: -50.0},
        Plot { x: -130.0, y: -50.0},
        Plot { x: -125.0, y: -50.0},
        Plot { x: -120.0, y: -50.0},
        Plot { x: -117.0, y: -55.0},
        Plot { x: -117.0, y: -60.0},
        Plot { x: -117.0, y: -65.0},
        Plot { x: -117.0, y: -70.0},
        Plot { x: -117.0, y: -75.0},
        Plot { x: -117.0, y: -80.0},
        Plot { x: -117.0, y: -85.0},
        Plot { x: -117.0, y: -90.0},
        Plot { x: -117.0, y: -95.0},
        Plot { x: -117.0, y: -100.0},
        Plot { x: -120.0, y: -100.0},
        Plot { x: -125.0, y: -100.0},
        Plot { x: -130.0, y: -100.0},
        Plot { x: -135.0, y: -100.0},
        Plot { x: -140.0, y: -100.0},
        Plot { x: -145.0, y: -100.0},
        Plot { x: -150.0, y: -100.0},
        Plot { x: -155.0, y: -100.0},
        Plot { x: -160.0, y: -100.0},
        Plot { x: -165.0, y: -100.0},
        //end of b

        //beginning of 5
        Plot { x: -30.0, y: 20.0},
        Plot { x: -35.0, y: 20.0},
        Plot { x: -40.0, y: 20.0},
        Plot { x: -45.0, y: 20.0},
        Plot { x: -50.0, y: 20.0},
        Plot { x: -55.0, y: 20.0},
        Plot { x: -60.0, y: 20.0},
        Plot { x: -65.0, y: 20.0},
        Plot { x: -70.0, y: 20.0},
        Plot { x: -75.0, y: 20.0},

        Plot { x: -75.0, y: 15.0},
        Plot { x: -75.0, y: 10.0},
        Plot { x: -75.0, y: 5.0},
        Plot { x: -75.0, y: 0.0},
        Plot { x: -75.0, y: -5.0},
        Plot { x: -75.0, y: -10.0},
        Plot { x: -75.0, y: -15.0},
        Plot { x: -75.0, y: -20.0},
        Plot { x: -75.0, y: -25.0},
        Plot { x: -75.0, y: -30.0},
        Plot { x: -75.0, y: -35.0},
        Plot { x: -75.0, y: -40.0},
        Plot { x: -75.0, y: -45.0},
        Plot { x: -75.0, y: -50.0},

        Plot { x: -75.0, y: -50.0},
        Plot { x: -70.0, y: -50.0},
        Plot { x: -65.0, y: -50.0},
        Plot { x: -60.0, y: -50.0},
        Plot { x: -55.0, y: -50.0},
        Plot { x: -50.0, y: -50.0},
        Plot { x: -45.0, y: -50.0},
        Plot { x: -40.0, y: -50.0},
        Plot { x: -35.0, y: -50.0},
        Plot { x: -30.0, y: -50.0},
        Plot { x: -30.0, y: -55.0},
        Plot { x: -30.0, y: -60.0},
        Plot { x: -30.0, y: -65.0},
        Plot { x: -30.0, y: -70.0},
        Plot { x: -30.0, y: -75.0},
        Plot { x: -30.0, y: -80.0},
        Plot { x: -30.0, y: -85.0},
        Plot { x: -30.0, y: -90.0},
        Plot { x: -30.0, y: -95.0},
        Plot { x: -30.0, y: -100.0},
        Plot { x: -30.0, y: -105.0},
        Plot { x: -35.0, y: -105.0},
        Plot { x: -40.0, y: -105.0},
        Plot { x: -45.0, y: -105.0},
        Plot { x: -50.0, y: -105.0},
        Plot { x: -55.0, y: -105.0},
        Plot { x: -60.0, y: -105.0},
        Plot { x: -65.0, y: -105.0},
        Plot { x: -70.0, y: -105.0},
        Plot { x: -75.0, y: -105.0},




        //start 8
        Plot { x:  10.0, y: 20.0},
        Plot { x:  15.0, y: 20.0},
        Plot { x:  20.0, y: 20.0},
        Plot { x:  25.0, y: 20.0},
        Plot { x:  30.0, y: 20.0},
        Plot { x:  35.0, y: 20.0},
        Plot { x:  40.0, y: 20.0},
        Plot { x:  45.0, y: 20.0},
        Plot { x:  50.0, y: 20.0},
        Plot { x:  55.0, y: 20.0},
        Plot { x:  60.0, y: 20.0},
        Plot { x:  65.0, y: 20.0},
        Plot { x:  70.0, y: 20.0},
        Plot { x:  70.0, y: 15.0},
        Plot { x:  70.0, y: 10.0},
        Plot { x:  70.0, y: 5.0},
        Plot { x:  70.0, y: 0.0},
        Plot { x:  70.0, y: -5.0},
        Plot { x:  70.0, y: -10.0},
        Plot { x:  70.0, y: -15.0},
        Plot { x:  70.0, y: -20.0},
        Plot { x:  70.0, y: -25.0},
        Plot { x:  70.0, y: -30.0},
        Plot { x:  70.0, y: -35.0},
        Plot { x:  70.0, y: -40.0},
        Plot { x:  70.0, y: -45.0},
        Plot { x:  65.0, y: -45.0},
        Plot { x:  60.0, y: -45.0},
        Plot { x:  55.0, y: -45.0},
        Plot { x:  50.0, y: -45.0},
        Plot { x:  45.0, y: -45.0},
        Plot { x:  40.0, y: -45.0},
        Plot { x:  35.0, y: -45.0},
        Plot { x:  30.0, y: -45.0},
        Plot { x:  25.0, y: -45.0},
        Plot { x:  20.0, y: -45.0},
        Plot { x:  15.0, y: -45.0},
        Plot { x:  10.0, y: -45.0},
        Plot { x:  10.0, y: -40.0},
        Plot { x:  10.0, y: -35.0},
        Plot { x:  10.0, y: -30.0},
        Plot { x:  10.0, y: -25.0},
        Plot { x:  10.0, y: -20.0},
        Plot { x:  10.0, y: -15.0},
        Plot { x:  10.0, y: -10.0},
        Plot { x:  10.0, y: -5.0},
        Plot { x:  10.0, y: 0.0},
        Plot { x:  10.0, y: 5.0},
        Plot { x:  10.0, y: 10.0},
        Plot { x:  10.0, y: 15.0},
        Plot { x:  10.0, y: 20.0},

        Plot { x:  10.0, y: -50.0},
        Plot { x:  10.0, y: -55.0},
        Plot { x:  10.0, y: -60.0},
        Plot { x:  10.0, y: -65.0},
        Plot { x:  10.0, y: -70.0},
        Plot { x:  10.0, y: -75.0},
        Plot { x:  10.0, y: -80.0},
        Plot { x:  10.0, y: -85.0},
        Plot { x:  10.0, y: -90.0},
        Plot { x:  10.0, y: -95.0},
        Plot { x:  10.0, y: -100.0},
        Plot { x:  10.0, y: -105.0},
        Plot { x:  15.0, y: -105.0},
        Plot { x:  20.0, y: -105.0},
        Plot { x:  25.0, y: -105.0},
        Plot { x:  30.0, y: -105.0},
        Plot { x:  35.0, y: -105.0},
        Plot { x:  40.0, y: -105.0},
        Plot { x:  45.0, y: -105.0},
        Plot { x:  50.0, y: -105.0},
        Plot { x:  55.0, y: -105.0},
        Plot { x:  60.0, y: -105.0},
        Plot { x:  65.0, y: -105.0},
        Plot { x:  70.0, y: -105.0},

        Plot { x:  70.0, y: -100.0},
        Plot { x:  70.0, y: -95.0},
        Plot { x:  70.0, y: -90.0},
        Plot { x:  70.0, y: -85.0},
        Plot { x:  70.0, y: -80.0},
        Plot { x:  70.0, y: -75.0},
        Plot { x:  70.0, y: -70.0},
        Plot { x:  70.0, y: -65.0},
        Plot { x:  70.0, y: -60.0},
        Plot { x:  70.0, y: -55.0},
        Plot { x:  70.0, y: -50.0},
        //finished 8       
    ];

    Model {
        land,
        window_id,
    }

}

fn view(app: &App, model: &Model, frame: Frame) {

    let r = random_f32() * 10.0;

    let draw = app.draw(); //initialize something I can draw onto. Canvas.

    draw
    .background()
    .color(BLACK); //background color of canvas I am drawing on



    //iterate through each plot in Model { land } vector and
    //draw plots on canvas. w_h() controls the plot size on canvas
    //x() and y() coordinate with location of plot.
    //color() changes plot color. 
    for plot in model.land.iter() {
        
        
        //generate a random floating point number between 0-10
        let alternate: u64 = random_range(0.0, 11.0) as u64;


        //match the value of the alternate variable.
        //each number is matched with a corresponding color type.
        let alternate_color = match alternate {
            0 => nannou::color::GREEN,
            1 => nannou::color::AZURE,
            2 => nannou::color::KHAKI,
            3 => nannou::color::CADETBLUE,
            4 => nannou::color::VIOLET,
            5 => nannou::color::REBECCAPURPLE,
            6 => nannou::color::FLORALWHITE,
            7 => nannou::color::OLDLACE,
            8 => nannou::color::YELLOW,
            9 => nannou::color::YELLOWGREEN,
            10 => nannou::color::ORCHID,
            _ => nannou::color::PALEGOLDENROD
        };

        //logic control for the moving stars. Checks x and y position
        //and if they are greater than provided values,
        //draw the specified shape.
        if plot.x > -500.0 && plot.y > 150.0 {
            draw.rect()
            .w_h(80.0, 1.0)
            .rotate(r * 5.0)
            .x(plot.x)
            .y(plot.y)
            .color(alternate_color);

        }


        //for everyother point not caught in the logic check
        draw.rect()
        .w_h(20.0, 7.0)
        .rotate(r * 5.0)
        .x(plot.x)
        .y(plot.y)
        .color(alternate_color);
        
    }

    draw
    .to_frame(app, &frame) //actually call the renderer to screen and show canvas in window.
    .unwrap() //crashes if renderer cant be created

}

fn update(app: &App, model: &mut Model, _update: Update) {

    //checks each plot,
    //logic checks to make sure plot if at the top of the screen
    //depending on position
    //change speed of moving Plot{} 
    for plot in model.land.iter_mut() {
        if plot.y >= 200.0 && plot.y < 229.0 {
            plot.x = plot.x + 3.0;
            if plot.x >= 500.0 {
                plot.x = -500.0
            }
        } else if plot.y >= 230.0 {
            plot.x = plot.x + 6.0;
            if plot.x >= 500.0 {
                plot.x = -500.0
            }
        }



    }
    //draw the update to canvas
    app.draw();
}

fn main() {


    nannou::app(init).update(update).run();
    

}
