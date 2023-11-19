use std::{io, fs, time::Duration, path, time::Instant};
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

enum SkitterFrame<'a> {
    Frame0(Vec<Span<'a>>),
    Frame1(Vec<Span<'a>>),
    Frame2(Vec<Span<'a>>),
    Frame3(Vec<Span<'a>>),
    Frame4(Vec<Span<'a>>),
    Frame5(Vec<Span<'a>>),
    Frame6(Vec<Span<'a>>),
    Frame7(Vec<Span<'a>>),
    Frame8(Vec<Span<'a>>),
    Frame9(Vec<Span<'a>>),
    Frame10(Vec<Span<'a>>),
    Frame11(Vec<Span<'a>>),
    Frame12(Vec<Span<'a>>),
    Frame13(Vec<Span<'a>>),
    Frame14(Vec<Span<'a>>),
    Frame15(Vec<Span<'a>>),
    Frame16(Vec<Span<'a>>),
    Frame17(Vec<Span<'a>>),
    Frame18(Vec<Span<'a>>),
    Frame19(Vec<Span<'a>>),
    Frame20(Vec<Span<'a>>),
    Frame21(Vec<Span<'a>>),
    Frame22(Vec<Span<'a>>),
    Frame23(Vec<Span<'a>>),
    Frame24(Vec<Span<'a>>),

}

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
    let mut vector_smash: [[[Vec3b ; 40]; 20]; 24] = [[[Vec3b::default(); 40]; 20]; 24];
    let mut n = 0;
    let mut have_skitter = false;
    let mut count_frame = 0;
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

                        let frame_row_name = format!("{}row{}",frame_name, numrow);
                        let mut col = 0;
                        for r in row.iter() {
                            let mut indicie = format!("{}col{}B", frame_row_name, col);
                            data[indicie] = r[0].into();
                            indicie = format!("{}col{}G", frame_row_name, col);
                            data[indicie] = r[1].into();
                            indicie = format!("{}col{}R", frame_row_name, col);
                            data[indicie] = r[2].into();
                            col += 1;


                        }
                        numrow = numrow + 1;

                    }
                    numrow = 0;
                    //increment the frame
                    num = num + 1;
                }
                //print out for debug purposes
                //println!("{}", data.dump());

                data_string = data.dump();
                //Now that we have the file, write it out
                let skitter_dir = fs::read_dir("./skitters").unwrap();
                let mut skitter_count = 0;
                for skit in skitter_dir {
                    skitter_count += 1;
                }
                let p_string = format!("./skitters/skitter{:03}", skitter_count);
                let path = path::Path::new(&p_string);

                fs::write(path, data_string).expect("file exists!");

                //keep the index from running on forever
                n = vector_smash.len() + 1;

                have_skitter = true;
                //break;

            }

        }
        if have_skitter {
            count_frame += 1;
            //println!("{}", data.dump());
            //break;
            if count_frame >= 24 {
                count_frame = 0;
            }

        }
            //imshow("doot", &ss).unwrap();

        if wait_key(5).unwrap() >= 0 {
            break;
        }





            //Draw the terminal
                terminal.hide_cursor()?;
                //terminal.clear()?;
                terminal.draw(|f| {
                    ui(f, 0, 0, vector_colours, now, count_frame);
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
                            ui(f, event.column, event.row, vector_colours, now, count_frame);
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

fn ui<B: Backend>(f: &mut Frame<B>, c: u16, r: u16, vector_colours: [[Vec3b; 40] ; 20], delta_time: Instant, count_frame: usize) {

    let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .margin(1)
    .constraints(
    [
        Constraint::Length(1),
                 Constraint::Length(41),
                 Constraint::Length(40),
                 Constraint::Length(40),
                 Constraint::Length(40),
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
    let mut num_skitter = 0;
    let skit_clone = skitter_dir.into_iter();
    for _skit in skit_clone {
        num_skitter += 1;
    }
    let skitter_dir = fs::read_dir("./skitters").unwrap();

    //let mut skitter_frames: SkitterFrame = <SkitterFrame<'_> as Default>::default();
    for skitters in skitter_dir {
        //println!("skitter: {:?}", skitters.unwrap().path().display());
        if num_skitter <= 0 {
            break;
        }
        let json_string = fs::read_to_string(skitters.as_ref().expect("No file!").path()).unwrap();
        let json_in = json::parse(&json_string).unwrap();
        let mut skitter_out = vec![];

        let fr: Vec<Span> = vec![];
        for frame in 0..34 {
            let frame_name = format!("frame{}", frame);
            for row in 0..20 {
                let frame_row_name = format!("{}row{}", frame_name, row);
                for col in 0..40 {
                    let frame_row_col_name_b = format!("{}col{}B", frame_row_name, col);
                    let frame_row_col_name_g = format!("{}col{}G", frame_row_name, col);
                    let frame_row_col_name_r = format!("{}col{}R", frame_row_name, col);
                    let b_u8 = json_in[frame_row_col_name_b].as_u8().expect("not a value!");
                    let g_u8 = json_in[frame_row_col_name_g].as_u8().expect("not a value!");
                    let r_u8 = json_in[frame_row_col_name_r].as_u8().expect("not a value!");
                    skitter_out.push(Span::styled( "A", Style::default().bg(Color::Rgb(r_u8, g_u8, b_u8))));
                }
                skitter_out.push(Span::raw("\n"));
            }
            /*
            let frame_skit_name = &format!("frame_{}", frame) as &str;
            match frame_skit_name {
                "frame_0" => skitter_frames.frame_0 = SkitterFrameIt { frame: skitter_out },
                "frame_1" => skitter_frames.frame_1 = SkitterFrameIt { frame: skitter_out },
                "frame_2" => skitter_frames.frame_2 = SkitterFrameIt { frame: skitter_out },
                "frame_3" => skitter_frames.frame_3 = SkitterFrameIt { frame: skitter_out },
                "frame_4" => skitter_frames.frame_4 = SkitterFrameIt { frame: skitter_out },
                "frame_5" => skitter_frames.frame_5 = SkitterFrameIt { frame: skitter_out },
                "frame_6" => skitter_frames.frame_6 = SkitterFrameIt { frame: skitter_out },
                "frame_7" => skitter_frames.frame_7 = SkitterFrameIt { frame: skitter_out },
                "frame_8" => skitter_frames.frame_8 = SkitterFrameIt { frame: skitter_out },
                "frame_9" => skitter_frames.frame_9 = SkitterFrameIt { frame: skitter_out },
                "frame_10" => skitter_frames.frame_10 = SkitterFrameIt { frame: skitter_out },
                "frame_11" => skitter_frames.frame_11 = SkitterFrameIt { frame: skitter_out },
                "frame_12" => skitter_frames.frame_12 = SkitterFrameIt { frame: skitter_out },
                "frame_13" => skitter_frames.frame_13 = SkitterFrameIt { frame: skitter_out },
                "frame_14" => skitter_frames.frame_14 = SkitterFrameIt { frame: skitter_out },
                "frame_15" => skitter_frames.frame_15 = SkitterFrameIt { frame: skitter_out },
                "frame_16" => skitter_frames.frame_16 = SkitterFrameIt { frame: skitter_out },
                "frame_17" => skitter_frames.frame_17 = SkitterFrameIt { frame: skitter_out },
                "frame_18" => skitter_frames.frame_18 = SkitterFrameIt { frame: skitter_out },
                "frame_19" => skitter_frames.frame_19 = SkitterFrameIt { frame: skitter_out },
                "frame_20" => skitter_frames.frame_20 = SkitterFrameIt { frame: skitter_out },
                "frame_21" => skitter_frames.frame_21 = SkitterFrameIt { frame: skitter_out },
                "frame_22" => skitter_frames.frame_22 = SkitterFrameIt { frame: skitter_out },
                "frame_23" => skitter_frames.frame_23 = SkitterFrameIt { frame: skitter_out },
                "frame_24" => skitter_frames.frame_24 = SkitterFrameIt { frame: skitter_out },

            }*/
//            skitter_frames.frame_skit_name.push(&mut SkitterFrameIt { frame: skitter_out },);

        }
//            let fr: Vec<_> = skitter_out.clone();
//            skitter_out = vec![];
            //for shown_frame in skitter_frames {
            //    let fr = Spans::from(shown_frame);
            //}
            let mut count = 0;
            //if count_frame == 0 {

                //for n in 0..num_skitter {
                    //if count == count_frame {




                        let fr = Spans::from(String::from("placeholder"));

                        let recording_block = Paragraph::new(fr)
                            .block(Block::default().title("Recorded").borders(Borders::ALL))
                            .alignment(Alignment::Center)
                            .wrap(Wrap {trim: true});
                        f.render_widget(recording_block, chunks[4]);

                    //}
                    count += 1;
                //};

            //}


        //}

        let mut count = 0;

        //if count_frame != 0 {
/*
            for n in 0..num_skitter {
                if n == count_frame {
                    let fr = Spans::from(skit.clone());

                    let recording_block = Paragraph::new(fr)
                        .block(Block::default().title("Recorded").borders(Borders::ALL))
                        .alignment(Alignment::Center)
                        .wrap(Wrap {trim: true});
                    f.render_widget(recording_block, chunks[4]);

                }
            };*/

        //}



        let elapsed_time = delta_time.elapsed();
        let timing = format!(":{} milliseconds have elapsed", elapsed_time.as_millis());
        let recording_block = Paragraph::new(Span::raw(timing))
            .block(Block::default().title("Timing").borders(Borders::ALL))
            .alignment(Alignment::Center)
            .wrap(Wrap {trim: true});
        f.render_widget(recording_block, chunks[3]);
    }



}
