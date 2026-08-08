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

PhaseNomad now combines the best of both worlds:

**Rust Core**: The `phase_nomad_core` library handles all the heavy lifting—API calls, data processing, and business logic. It's compiled into an XCFramework that the iOS app can use seamlessly.

**Swift UI**: The iOS app provides an elegant interface for managing flight alerts, viewing results, and configuring preferences. SwiftUI made it possible to build a modern, responsive interface without the complexity of UIKit.

**Background Processing**: iOS background tasks ensure your flight alerts stay updated even when you're not actively using the app. No more cron jobs or server maintenance.

The app features everything the original automation had and more:
- Set up flight alerts with specific criteria (dates, prices, cabin class)
- Background processing to check for new deals
- Clean, organized results with direct booking links
- No emails cluttering your inbox—just check the app when convenient

## Lessons Learned

Looking back at this evolution, a few things stand out:

**Start Simple**: The Google Sheets automation was crude, but it validated the idea. Don't over-engineer from the start.

**Rust for Performance**: When I needed reliability and speed, Rust delivered. The same core logic that powered the CLI now runs on many iOS devices.

**User Experience Matters**: The most powerful tool is useless if it's hard to use. The iOS app made flight tracking accessible in a way the CLI never could.

**UniFFI is Magic**: Being able to write performance-critical code in Rust and wrap it in a beautiful Swift interface is incredibly powerful. It's the best of both ecosystems.

## What's Next?

PhaseNomad is now available on the App Store, and I'm constantly surprised by how much I use it myself. Instead of manually checking flight prices, I set up alerts and let the app work in the background. When I open it, I see exactly what I need: flights that match my criteria at prices I'm willing to pay.

The journey from spreadsheet to app store taught me that sometimes the best solutions evolve gradually. Each phase built on the previous one, solving real problems along the way. The Google Sheets automation was clunky but functional. The Rust CLI was powerful but limited. The iOS app finally made flight tracking effortless.

For anyone building their own automation projects, remember: start with what works, iterate based on real usage, and don't be afraid to completely rewrite when the benefits are clear. Sometimes the best architecture is the one that evolves naturally from solving real problems.

Now, if you'll excuse me, I need to check if there are any good deals for my next adventure. Thankfully, I have an app for that.

[PhaseNomad on the App Store](https://apps.apple.com/gb/app/phasenomad/id6745560782)

[PhaseNomad offical site](https://phasenomad.app)
