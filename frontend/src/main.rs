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
use opencv::core::{Mat, CV_8U, Point, Vector, Vec3b, MatIter};
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

            let mut vector_colours: [[Vec3b ;40]; 40] = [[Vec3b::default(); 40]; 40];

            if cam_ok == false {
                println!("failed opening the VideoCapture");
                break;
            }

            let err = cap.read(&mut frame);
            if err.is_ok() {
                let ssize = ss.size().unwrap();
                let err = resize(&frame, &mut ss, ssize, 0.0, 0.0, INTER_AREA);
                err.unwrap();
                let mut col_num = 0;
                let mut r = 0;
                let mut c = 0;
                for row in 0..40 {
                    //println!("{:?}", ss.row(row));

                    for col in 0..40 {

                        let dat: Vec3b = *ss.at_2d(r, c).expect("Out of bounds!");
                        //get the first blue pixel, print it out, then quit
                        let ur = r as usize;
                        let uc = c as usize;
                        vector_colours[ur][uc] = dat;
                        //print!("{:?}", dat[0]);
                        //TODO add a way to collect the values || or act directly on the values now
                        //break;
                        c += 1;

                    }
                    c = 0;
                    r += 1;
                    //break;
                }
                break;

                //imshow("doot", &ss).unwrap();
            }
            break;

            if wait_key(5).unwrap() >= 0 {
                break;
            }





            //Draw the terminal
                terminal.hide_cursor()?;
                terminal.clear()?;
                terminal.draw(|f| {
                    ui(f, 0, 0, vector_colours);
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
                            ui(f, event.column, event.row, vector_colours);
                        })?;

                    },
                    Event::FocusGained => println!("Stole focus!"),
                    Event::FocusLost => println!("Lost focus!"),
                    Event::Paste(data) => println!("{:?}", data),
                    Event::Resize(width, height) => println!("New Size {}x{}", width, height),
                }
            }
        }//end loop
    }//end unsafe




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

fn ui<B: Backend>(f: &mut Frame<B>, c: u16, r: u16, vector_colours: [[Vec3b; 40] ; 40]) {
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
    let mut cam_span = vec![];

        for r in 0..40 {

                for c in 0..40 {

                    cam_span.push(Span::styled(" ", Style::default().bg(Color::Rgb(vector_colours[r][c][2], vector_colours[r][c][1], vector_colours[r][c][0]))));
                }
                cam_span.push(Span::raw("\n"));
            };


    let block = Paragraph::new(Spans::from(cam_span))
    .block(Block::default()
    .title("Block 2")
    .borders(Borders::ALL));
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
