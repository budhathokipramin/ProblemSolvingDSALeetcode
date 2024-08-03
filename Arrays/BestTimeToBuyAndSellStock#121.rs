// You are given an array prices where prices[i] is the price of a given stock on the ith day.

// You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.

// Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // If the prices array is empty, return 0
    if prices.is_empty() {
        return 0;
    }

    // Initialize min_price to the first element and max_profit to 0
    let mut min_price = prices[0];
    let mut max_profit = 0;

    // Iterate through the array
    for &price in &prices {
        // Update min_price if the current price is lower
        if price < min_price {
            min_price = price;
        }

        // Calculate the current profit
        let profit = price - min_price;

        // Update max_profit if the current profit is higher
        if profit > max_profit {
            max_profit = profit;
        }
    }

    return max_profit        
    }
}
