use plotters::prelude::*;
use rand::Rng;

const SIMULATION_DAYS: usize = 365;
const INITIAL_PRICE: f64 = 100.0;
const STOP_LOSS_PERCENT: f64 = 0.01;
const TAKE_PROFIT_PERCENT: f64 = 0.03;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut balance = 0.0;
    let mut balance_history = Vec::with_capacity(SIMULATION_DAYS);
    let mut price_history = Vec::with_capacity(SIMULATION_DAYS);
    let mut rng = rand::thread_rng();

    let mut price = INITIAL_PRICE;
    let mut day = 0;
    let mut stop_loss: f64 = 0.0;
    let mut take_profit: f64 = 0.0;

    make_new_order(&mut stop_loss, price, &mut take_profit);

    while day < SIMULATION_DAYS {
        day += 1; // Move to the next day
        let change_percent = rng.gen_range(-0.03..0.03); // Random change between -5% and 5%
        price *= 1.0 + change_percent;
        price_history.push(price); // Save the price for the graph

        if price <= stop_loss {
            balance -= INITIAL_PRICE * STOP_LOSS_PERCENT;
            make_new_order(&mut stop_loss, price, &mut take_profit);
        } else if price >= take_profit {
            balance += INITIAL_PRICE * TAKE_PROFIT_PERCENT;
            make_new_order(&mut stop_loss, price, &mut take_profit);
        }
        balance_history.push(balance);
    }

    // Fill the rest of the days if the last trade didn't reach stop-loss or take-profit
    while balance_history.len() < SIMULATION_DAYS {
        balance_history.push(balance);
        price_history.push(price); // Assume the price stays the same for simplicity
    }

    // Draw the balance history graph
    let root_area = BitMapBackend::new("balance_history.png", (640, 480)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let min_balance = *balance_history
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_balance = *balance_history
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_price = *price_history
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let max_y = max_balance.max(max_price);

    let mut chart = ChartBuilder::on(&root_area)
        .caption(
            "Trader Balance and Asset Price History",
            ("sans-serif", 40).into_font(),
        )
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_ranged(0u32..SIMULATION_DAYS as u32, min_balance..max_y)?;

    chart.configure_mesh().draw()?;

    // Draw balance line
    chart
        .draw_series(LineSeries::new(
            (0..SIMULATION_DAYS as u32).zip(balance_history.into_iter()),
            &RED,
        ))?
        .label("Balance")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // Draw price line
    chart
        .draw_series(LineSeries::new(
            (0..SIMULATION_DAYS as u32).zip(price_history.into_iter()),
            &BLUE,
        ))?
        .label("Asset Price")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // Draw legend
    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .draw()?;

    println!("Simulation finished. Balance history graph is saved as 'balance_history.png'.");

    Ok(())
}

fn make_new_order(stop_loss: &mut f64, price: f64, take_profit: &mut f64) {
    *stop_loss = price * (1.0 - STOP_LOSS_PERCENT);
    *take_profit = price * (1.0 + TAKE_PROFIT_PERCENT);
}
