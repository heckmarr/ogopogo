use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Alignment, Direction, Layout},
    widgets::{Paragraph, Block, Borders, Wrap},
    style::{Color, Style, Modifier},
    text::{Span,Spans},
    terminal::Terminal,
    Frame,
};
use std::{io, time::Duration};
use crossterm:: {
    event::{read, poll, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use opencv::prelude::*;
use opencv::videoio;
use opencv::core::{Mat, CV_8U, Point};
use opencv::imgproc::{resize, INTER_AREA};
use opencv::highgui::{imshow, wait_key};

fn main() -> Result<(), io::Error> {

    //Initialize the terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    let mut cap = videoio::VideoCapture::default().unwrap();
    let cam_ok = videoio::VideoCapture::open(&mut cap, 0, videoio::CAP_ANY).unwrap();

    let mut frame = Mat::default();
    let mut ss = Mat::default();
    unsafe {

        let _shrunken_ok = Mat::create_rows_cols(&mut ss, 40, 40, CV_8U);

        //loop and poll for events
        loop {


            if cam_ok == false {
                println!("failed opening the VideoCapture");
                break;
            }

            let err = cap.read(&mut frame);
            if err.is_ok() {
                let ssize = ss.size().unwrap();
                //println!("{:?}", ssize);

                let err = resize(&frame, &mut ss, ssize, 0.0, 0.0, INTER_AREA);
                err.unwrap();

//                let v: Vec<u8> = ss.iter().flat_map(|v| v.iter()).cloned().collect();
                let two_dee: Vec<Vec<(u8, u8, u8)>>= ss.to_vec_2d().unwrap();
                let mut col_num = 0;
                for row in 0..40 {
                    //r += 1;
                    println!("{:?}", ss.row(row));

                    for col in 0..40 {
                        col_num += 1;
                        let row_pos: usize = (row+(col * col_num)) as usize;
                        //let row_it = ss.row(row).unwrap().col(col).unwrap();
                        let row_it = &two_dee[row_pos];
                        //let ss_snake = row_it.clone();
                        //                        let row_at: Vec<_> = ss_snake.at<Vec<_>>(row + col*col_num).expect("Not a colour!");
                        //let row_pt: Point = Point {y: row, x:col*col_num};
                        println!("{:?}", row_it);

                        break;
                    }
                    break;
                }
                break;

                //let v: Vec<u8> = ss.iter().flat_map(|v| v.iter()).cloned().collect();
                //println!("{:?}", v);
                //imshow("doot", &ss).unwrap();
            }
            break;
            if wait_key(5).unwrap() >= 0 {
                break;
            }




/*
            //Draw the terminal
                terminal.hide_cursor()?;
                terminal.clear()?;
                terminal.draw(|f| {
                    ui(f, 0, 0);
                })?;

            if poll(Duration::from_millis(10))? {
                //Poll for events and match them to a read case
                match read()? {
                    Event::Key(_event) => break,
                    Event::Mouse(event) => {//break out the case code
                        let stdout = io::stdout();
                        let backend = CrosstermBackend::new(stdout);
                        let mut terminal = Terminal::new(backend)?;
                        terminal.draw(|f| {
                            ui(f, event.column, event.row);
                        })?;

                    },
                    Event::FocusGained => println!("Stole focus!"),
                    Event::FocusLost => println!("Lost focus!"),
                    Event::Paste(data) => println!("{:?}", data),
                    Event::Resize(width, height) => println!("New Size {}x{}", width, height),
                }
            }*/
        }//end loop
    }//end unsafe




    //restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, c: u16, r: u16) {
    let chunks = Layout::default()
    .direction(Direction::Vertical)
    .margin(1)
    .constraints(
    [
        Constraint::Percentage(10),
                 Constraint::Percentage(60),
                 Constraint::Percentage(30)
    ].as_ref()
    )
    .split(f.size());
    let block = Block::default()
    .title("Block 1")
    .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    let block = Block::default()
    .title("Block 2")
    .borders(Borders::ALL);
    f.render_widget(block, chunks[1]);

    let span_styled = format!("And only a test, cursor is at {}cx{}r Any key to end", c, r);
    let text = vec![
        Spans::from(vec![
            Span::raw("This is a "),
            Span::styled("test", Style::default().add_modifier(Modifier::ITALIC)),
            Span::raw("."),
        ]),
        Spans::from(Span::styled(span_styled, Style::default().fg(Color::Red))),
    ];
    let final_block = Paragraph::new(text)
        .block(Block::default().title("Paragraph").borders(Borders::ALL))
        .alignment(Alignment::Center)
        .wrap(Wrap {trim: true});
    f.render_widget(final_block, chunks[2]);

}
