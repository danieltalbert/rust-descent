// predict: the classic y = mx + b. Hand it the line and a pizza size, it hands back the guessed price.
fn predict(slope: f64, intercept: f64, diameter: f64) -> f64 {
    slope * diameter + intercept
}


// pizza_data: the dataset. Ten pizzas with their sizes and real prices, the one source of truth every other function borrows.
fn pizza_data() -> Vec<(f64, f64)> {
    vec![
        (6.0, 7.0),
        (8.0, 9.0),
        (10.0, 11.0),
        (12.0, 13.5),
        (14.0, 15.0),
        (16.0, 17.5),
        (18.0, 20.0),
        (20.0, 22.0),
        (22.0, 24.5),
        (24.0, 27.0),
    ]
}

// main: the conductor. It loads the data, trains the line, prints the guesses, the final cost, the answer key check, and the plot, in that order.
fn main() {
    let slope = 1.0;
    let intercept = 1.0;
    let data = pizza_data();
    let (slope, intercept) = train(slope, intercept, &data, 0.002, 5000);
    let (key_slope, key_intercept) = closed_form(&data);


    for point in &data {
        let guess = predict(slope, intercept, point.0);
        println!("{} in: real ${}, guessed ${:.2}", point.0, point.1, guess);
    }
    println!("Final cost: {:.4}", cost(slope, intercept, &data));
    println!("Answer key: slope {:.4}, intercept {:.4}, cost {:.4}", key_slope, key_intercept, cost(key_slope, key_intercept, &data));
    print_plot(slope, intercept, &data);

}

// print_plot: the picture. Stars are real pizzas, dashes are my line running through them, so you can see the fit instead of just trusting numbers.
fn print_plot(slope: f64, intercept: f64, data: &[(f64, f64)]) {
    for row in (0..=13).rev() {
        let band_low = row as f64 * 2.0;
        let mut line = String::new();
        for col in 0..=18 {
            let diameter = 6.0 + col as f64;
            let mut mark = ' ';
            let guess = predict(slope, intercept, diameter);
            if guess >= band_low && guess < band_low + 2.0 {
                mark = '-';
            }
            for point in data {
                if point.0 == diameter && point.1 >= band_low && point.1 < band_low + 2.0 {
                    mark = '*';
                }
            }
            line.push(mark);
        }
        println!("{:>4} |{}", band_low, line);
    }
}

// cost: the scorekeeper. It squares every miss so highs and lows can't cancel each other out, then averages them into one "how wrong is this line" number.
fn cost(slope: f64, intercept: f64, data: &[(f64, f64)]) -> f64 {
    let mut total = 0.0;
    for point in data {
        let error = predict(slope, intercept, point.0) - point.1;
        total += error * error;
    }
    total / data.len() as f64
}

// gradient: the compass. It tells each knob which way is downhill and how steep it is, and for the slope knob each miss counts extra depending on the pizza's size.
fn gradient(slope: f64, intercept: f64, data: &[(f64, f64)]) -> (f64, f64) {
    let mut slope_sum = 0.0;
    let mut intercept_sum = 0.0;
    for point in data {
        let error = predict(slope, intercept, point.0) - point.1;
        slope_sum += error * point.0;
        intercept_sum += error;
    }
    let n = data.len() as f64;
    (2.0 / n * slope_sum, 2.0 / n * intercept_sum)
}

// train: the learning loop. Ask the compass, take a small step downhill, repeat 5000 times until the line settles at the bottom.
fn train(slope: f64, intercept: f64, data: &[(f64, f64)], learning_rate: f64, epochs: usize) -> (f64, f64) {
    let mut slope = slope;
    let mut intercept = intercept;
    for _ in 0..epochs {
        let (g_slope, g_intercept) = gradient(slope, intercept, data);
        slope -= learning_rate * g_slope;
        intercept -= learning_rate * g_intercept;
    }
    (slope, intercept)
}

// closed_form: the answer key. It solves for the perfect line directly with the formula, so I can prove the learned line actually got there.
fn closed_form(data: &[(f64, f64)]) -> (f64, f64) {
    let n = data.len() as f64;
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    for point in data {
        sum_x += point.0;
        sum_y += point.1;
    }
    let mean_x = sum_x / n;
    let mean_y = sum_y / n;
    let mut top = 0.0;
    let mut bottom = 0.0;
    for point in data {
        top += (point.0 - mean_x) * (point.1 - mean_y);
        bottom += (point.0 - mean_x) * (point.0 - mean_x);
    }
    let slope = top / bottom;
    (slope, mean_y - slope * mean_x)
}