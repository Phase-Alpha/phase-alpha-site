---
title: From Spreadsheets to App Store - The PhaseNomad Journey
description: How a simple flight tracking idea evolved from Google Sheets automation to a Rust-powered iOS app
date: 2025-07-10
image_path: '/phase_nomad_evolution.jpeg'
tags: [tech]
---

Sometimes the best projects start with the simplest annoyances. For me, it was manually checking flight prices across multiple websites, trying to find the perfect deal for my next adventure. What began as a basic Google Sheets automation script eventually evolved into PhaseNomad, a full-featured iOS app that's now available on the App Store.

## The Google Sheets Era

Like many automation projects, this one started with a spreadsheet. I had a simple Google Sheets setup where I'd manually track flight prices between my favorite destinations. The process was tedious: copy-paste URLs, update prices, try to spot trends. It worked, but barely.

Being an engineer, I couldn't resist the urge to automate. I wrote a Python script that would pull flight data from various APIs and populate my spreadsheet automatically. It was clunky, but it worked. The script would run on a schedule, updating prices and sending me a text alert with Twilio when it found good deals.

## The Rust CLI Phase

After living with the Python automation for a while, I realized I needed something more robust. The Python script was fragile, API changes would break it, and maintaining all those dependencies was becoming a headache. This seemed like the perfect opportunity to explore Rust as an option.

I rewrote the core logic in Rust, creating a CLI tool that could fetch flight data from the Kiwi.com Tequila API. The performance difference and simpilicity was immediately noticeable. What used to take longer in Python and depended on 2 different services now ran in seconds, and only needed my server to be up. The CLI was solid, reliable, and I could run it on cron jobs without worrying about it breaking.

```rust
// The core function that started it all
pub fn search_flights(
    api_key: &str,
    request: &FlightRequest,
) -> Result<Vec<FlightResult>, FlightError> {
    // Flight searching logic
}
```

The Rust version was everything I wanted: fast, reliable, and memory-efficient. But it was still just a command-line tool sending emails. I found myself wanting more, better visualizations, easier management of my flight alerts, and the ability to check results on the go.

## The iOS Evolution

The jump from CLI to mobile app wasn't immediate. I spent time thinking about whether this was worth the effort. Could I really build something better than just checking flight websites directly?

The answer became clear when I started using UniFFI to bridge my Rust logic with Swift. Suddenly, I had the best of both worlds: the performance and reliability of Rust for the core flight processing, wrapped in a beautiful SwiftUI interface, not to mention I could re-use a lot of what I already had.

```swift
// Bridging Rust and Swift with UniFFI
let result = try PhaseNomadCore.fetchFlightData(
    apiKey: apiKey, 
    request: requestPayload
)
```

Building the iOS app taught me a lot about user experience. The CLI was powerful but intimidating. The mobile app needed to be intuitive, something you could set up in minutes and check casually without thinking about API keys or command-line arguments.

## The Architecture Today

PhaseNomad now runs on both halves of that history. The `phase_nomad_core` library handles the API calls, data processing and business logic, compiled into an XCFramework that the iOS app links against. On top of that sits a SwiftUI interface for managing alerts, viewing results and configuring preferences, which was considerably less work than the equivalent in UIKit would have been. iOS background tasks keep the alerts updated while the app is closed, so there are no cron jobs or servers left to maintain.

The app does everything the original automation did:
- Set up flight alerts with specific criteria (dates, prices, cabin class)
- Background processing to check for new deals
- Clean, organized results with direct booking links
- No emails cluttering your inbox, just open the app when it suits you

## Lessons Learned

The Google Sheets version was crude, but it proved the idea was worth chasing, and I'd have wasted months if I'd tried to build the app first. Rust earned its place the moment I needed the thing to stop breaking, and that same core logic now runs on every device the app is installed on.

The part I underestimated was the interface. The CLI was more capable than the app in raw terms and I still checked it less often, purely because it was a faff. UniFFI is what made the combination possible, and writing the performance sensitive code once and wrapping it in Swift is the thing I'd take from this project into the next one.

## What's Next?

PhaseNomad is now available on the App Store, and I'm constantly surprised by how much I use it myself. Instead of manually checking flight prices, I set up alerts and let the app work in the background. When I open it, I see exactly what I need: flights that match my criteria at prices I'm willing to pay.

None of this was planned. Each version existed because the previous one annoyed me enough to replace it, and every rewrite only made sense once that annoyance was obvious. That's the one thing I'd pass on: build the crude version first and let it tell you what's wrong with itself.

Now, if you'll excuse me, I need to check if there are any good deals for my next adventure. Thankfully, I have an app for that.

[PhaseNomad on the App Store](https://apps.apple.com/gb/app/phasenomad/id6745560782)

[PhaseNomad offical site](https://phasenomad.app)
