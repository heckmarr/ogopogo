use std::{io, fs, time::Duration, time::Instant};
//Tui imports
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Alignment, Direction, Layout},
    widgets::{Paragraph, Block, Borders, Wrap},
    style::{Color, Style, Modifier},
    text::{Span,Spans},
    terminal::Terminal,
    Frame,
};
use crossterm:: {
    event::{read, poll, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
//opencv imports
use opencv::prelude::*;
use opencv::videoio;
use opencv::core::{Mat, CV_8U, Vec3b};
use opencv::imgproc::{resize, INTER_AREA};
use opencv::highgui::{wait_key};
//Rabbitmq imports




fn main() -> Result<(), io::Error> {

    //Initialize the terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout,
             EnterAlternateScreen,
             EnableMouseCapture
             )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    let mut cap = videoio::VideoCapture::default().unwrap();
    let cam_ok = videoio::VideoCapture::open(&mut cap, 0, videoio::CAP_ANY).unwrap();

    let mut frame = Mat::default();
    let mut ss = Mat::default();
    unsafe {let _shrunken_ok = Mat::create_rows_cols(&mut ss, 20, 40, CV_8U);};
    terminal.clear()?;
    //loop and poll for events
    let mut vector_smash: [[[Vec3b ; 40]; 20]; 35] = [[[Vec3b::default(); 40]; 20]; 35];
    let mut n = 0;
    let mut exit = false;
    let mut data_string = "".into();
    let mut data = json::JsonValue::new_object();
    loop {
        let now = Instant::now();
        let mut vector_colours: [[Vec3b ;40]; 20] = [[Vec3b::default(); 40]; 20];

        if cam_ok == false {
            println!("failed opening the VideoCapture");
            break;
        }

        let err = cap.read(&mut frame);
        if err.is_ok() {
            let ssize = ss.size().unwrap();
            let err = resize(&frame, &mut ss, ssize, 0.0, 0.0, INTER_AREA);
            err.unwrap();

            let mut r = 0;
            let mut c = 0;
            for _row in 0..20 {
                //println!("{:?}", ss.row(row));

                for _col in 0..40 {

                    let dat: Vec3b = *ss.at_2d(r, c).expect("Out of bounds!");
                    //get the first blue pixel, print it out, then quit
                    let ur = r as usize;
                    let uc = c as usize;
                    vector_colours[ur][uc] = dat;
                    //break;
                    c += 1;

                }
                c = 0;
                r += 1;
                //break;
            }
            //break;
            if n < vector_smash.len() {
                vector_smash[n] = vector_colours;
            }
            n = n + 1;
            if n == vector_smash.len() - 1 {
                let mut num = 0;
                let mut numrow = 0;
                for frame in vector_smash.iter() {
                    let frame_name = format!("frame{}", num);
                    //loop over the frames
                    for row in frame.iter() {
                        //loop over the rows
                        numrow = numrow + 1;
                        let frame_row_name = format!("{}row{}",frame_name, numrow);
                        let mut col = 0;
                        for r in row.iter() {
                            col += 1;
                            let mut indicie = format!("{}col{}B", frame_row_name, col);
                            data[indicie] = r[0].into();
                            indicie = format!("{}col{}G", frame_row_name, col);
                            data[indicie] = r[1].into();
                            indicie = format!("{}col{}R", frame_row_name, col);
                            data[indicie] = r[2].into();
                        }

                    }
                    numrow = 0;
                    //increment the frame
                    num = num + 1;
                }
                //print out for debug purposes
                //println!("{}", data.dump());

                data_string = data.dump();
                //keep the index from running on forever
                n = vector_smash.len() + 1;

                exit = false;
                //break;

            }

        }
        if exit {
            //println!("{}", data.dump());
            break;

        }
            //imshow("doot", &ss).unwrap();

        if wait_key(5).unwrap() >= 0 {
            break;
        }





            //Draw the terminal
                terminal.hide_cursor()?;
                //terminal.clear()?;
                terminal.draw(|f| {
                    ui(f, 0, 0, vector_colours, &data_string, now);
                })?;
//                println!("{}", data_string);

            if poll(Duration::from_millis(10))? {
                //Poll for events and match them to a read case
                match read()? {
                    Event::Key(_event) => break,
                    Event::Mouse(event) => {//break out the case code
                        let stdout = io::stdout();
                        let backend = CrosstermBackend::new(stdout);
                        let mut terminal = Terminal::new(backend)?;
                        terminal.draw(|f| {
                            ui(f, event.column, event.row, vector_colours, &data_string, now);
                        })?;

                    },
                    Event::FocusGained => println!("Stole focus!"),
                    Event::FocusLost => println!("Lost focus!"),
                    Event::Paste(data) => println!("{:?}", data),
                    Event::Resize(width, height) => println!("New Size {}x{}", width, height),
                }
            }
        }//end loop




    //restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        // TODO remove this comment
        //LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, c: u16, r: u16, vector_colours: [[Vec3b; 40] ; 20], data_string: &str, delta_time: Instant) {




    let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .margin(1)
    .constraints(
    [
        Constraint::Length(1),
                 Constraint::Length(41),
                 Constraint::Length(40),
                 Constraint::Percentage(30)
    ].as_ref()
    )
    .split(f.size());
    let block = Block::default()
    .title("Block 1")
    .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    let mut cam_span = vec![];

        for r in 0..20 {

                for c in 0..40 {
                    //looks like this will work for reproducing the image

                    //I just need to get the timing so it will trigger at 33 ms at least.

                    //The file will need to be loaded as well, which adds to the overall timing
                    cam_span.push(Span::styled(" ", Style::default().bg(Color::Rgb(vector_colours[r][c][2], vector_colours[r][c][1], vector_colours[r][c][0]))));
                }
                cam_span.push(Span::raw("\n"));
            };


    let block = Paragraph::new(Spans::from(cam_span))
    .block(Block::default()
    .title("LIVE")
    .borders(Borders::ALL)).wrap(Wrap {trim: false});
    f.render_widget(block, chunks[1]);

    let span_styled = format!("And only a test, cursor is at {}cx{}r Any key to end", c, r);

    let text_info = vec![
        Spans::from(vec![
            Span::raw("This is a "),
            Span::styled("test frame, of half of my head most likely, standing at my desk", Style::default().add_modifier(Modifier::ITALIC)),
                    Span::raw("."),


        ]),
        Spans::from(Span::styled(span_styled, Style::default().fg(Color::Red))),
    ];


    let text_block = Paragraph::new(text_info)
        .block(Block::default().title("Info").borders(Borders::ALL))
        .alignment(Alignment::Center)
        .wrap(Wrap {trim: true});
    f.render_widget(text_block, chunks[2]);


    //read the number of stored images
    let skitter_dir = fs::read_dir("./skitters").unwrap();
    for skitters in skitter_dir {
        //println!("skitter: {:?}", skitters.unwrap().path().display());

        let json_string = fs::read_to_string(skitters.as_ref().expect("No file!").path()).unwrap();
        let json_in = json::parse(&json_string).unwrap();
        let mut skitter_out = vec![];
        let mut skitter_frames = vec![];
        for frame in 0..34 {
            let frame_name = format!("{}frame", frame);
            for row in 0..20 {
                let frame_row_name = format!("{}row{}", frame_name, row);
                for col in 0..40 {
                    let frame_row_col_name_b = format!("{}col{}B", frame_row_name, col);
                    let frame_row_col_name_g = format!("{}col{}G", frame_row_name, col);
                    let frame_row_col_name_r = format!("{}col{}R", frame_row_name, col);
                    let b_u8: u8 = json_in[frame_row_col_name_b].as_u8().unwrap();
                    let g_u8: u8 = json_in[frame_row_col_name_g].as_u8().unwrap();
                    let r_u8: u8 = json_in[frame_row_col_name_r].as_u8().unwrap();
                    skitter_out.push(Span::styled( " ", Style::default().bg(Color::Rgb(r_u8, g_u8, b_u8))));
                }
            }
            skitter_frames.push(skitter_out.clone());
            skitter_out = vec![];
        }


        let elapsed_time = delta_time.elapsed();
        let timing = format!(":{} milliseconds have elapsed", elapsed_time.as_millis());
        let recording_block = Paragraph::new(Span::raw(timing))
            .block(Block::default().title("Recorded").borders(Borders::ALL))
            .alignment(Alignment::Center)
            .wrap(Wrap {trim: true});
        f.render_widget(recording_block, chunks[3]);
    }



}
