
# Writeup:
## Introduction
This project started from the simple desire for me to come up with my own concept for a hardware badge, akin to the ones you'd see at places like [DefCon](https://defcon.org/) or [SuperCon](https://hackaday.com/tag/supercon/). If you are not aware, hardware badges are generally thematic, bespoke development boards with included hardware/peripherals that are easy to hack and modify. To me, the hardware badge is the perfect manifestation of the maker/hacker culture. They embody the curiosity to experiment and try new things, the desire to always be learning and improving, and most importantly the willingness and desire to share your project with the community so others can be inspired, learn from, and subsequently build upon your work. This last facet is the most important to me because it's absolutely the foundation of the maker community. It is from this sense of sharing with community that really drove me really stop "lurking" and start putting my own projects out for others to see.

At first for me, it was a matter of getting more organized and collecting my various existing project files into public Git repos, and I definitely still have more work to do here. But I also really value all of the project writeups I've read over the years from the multitude of others who've taken the time to both share their projects, but also share their process, struggles, learnings, etc. So it with that spirit that I decided to share my own projects experiences in this writeup. Writing has never really been enjoyable for me, mostly because I feel so inefficient when I do. As a side benefit, I am also hoping this writeup will serve as some great training data to perhaps help me write more writeups, more efficiently, in the future.

## Design Inspiration And Concept
The overall design idea of Light Rail came together in my head unexpectedly, all at once. I was in bed, trying to wrangle my thoughts and fall asleep. My mental state was still buzzing with the resolve to finally come up with an idea for my own hardware badge. For months prior, I had been toying around with small projects, experimenting with different breakout boards for LED drivers, trying out many small ideas like simple animations for sensor data, etc. Then that night, I was looking at all of my projects where I have them displayed (a collection, not a hoard) when a thought afflicted me--all of these projects are running your code, your ideas manifested, but none of it is "your" hardware. With that realization, I decided to think up and design my own unique take on a hardware badge. So right after I made my way to bed with a racing mind, I did find eventually my way to a restful state and stop the train of thoughts. However, the calm in my head only lasted a few moments; the gears of my subconscious must have kept spinning. Before I was able to fall alseep, in an instant, all of the chaotic swirling of all my earlier scattered thoughts came rushing back. But then, surprisingly, choas of ideas proceeed to swiftly coalesce in my head with the precision, speed, and grace of a team of animal mechs assembling into Voltron. 

It was really quite an experience for me! I wanted to give a non-exhustive list of the sorts of ideas and notions I was thinking about when everything coalascked:
- trains, in general
- Lego trains (this is not my first train project! Also see my [M5Cardputer firmware](https://github.com/nonik0/CardputerLegoTrainControl) for controlling Lego trains and switches)
- my nephew who also loves trains
- [Mini Metro](https://dinopoloclub.com/games/mini-metro/), a game with trains
- PCB Convention Badges, especially [Supercon 2022 badge](https://github.com/Hack-a-Day/2022-Supercon6-Badge-Tool)
- [TIGER Electronics](https://en.wikipedia.org/wiki/Tiger_Electronics) handheld LCD games
- Adafruit LED backpacks, especially [this one](https://www.adafruit.com/product/2946)

[TODO: picture of sketch]

OK, so what was the actual idea? It was to be a train game. The tracks would be composed of lines of LEDs, and then trains would be the lit LEDs that would move along the tracks, each LED like a car in the train. Along the LED tracks, there would other LEDs that would be platforms. When lit, the platform has cargo that trains could pick up from the adjacent track and deliver to another platform. The train could show its cargo with brightly lit full cars and dimly lit empty cars. The track would have several forks and crosses with push button next to each to toggle/switch the fork or cross. Later, I had the idea that all of the board components could contribute to the design by posing as various buildings or structures, and I drew silkscreen roads and parking spaces to help perceive this notion.

As far as the gameplay, I never really got too far, and to be frank it's the least important part of the project for me. I am very happy with the fact that the board itself gives me a lot of gameplay ideas to experiment with and try. But I haven't done any yet, that's for part 2!

## Hardware Design
### Design Goals
As soon as the Light Rail idea came into my head, I started right away. As I mentioned earlier, I had been trying to sleep but, after the idea came up, had gotten up immediately to start chugging away at the initial schematic in Kicad. By the time I went back to bed, I had the majority of what would be the final schematic complete! Since my original vision, not much has been altered. Somewhat surprisingly to me though, was how design process "felt". From beginnging to end, it felt to me much more like unveiling the breadth of the idea in my head than coming up with it piecewise through the process. That's the best way I can think to explain my feeling in words. But despite this strong vision, I really found a ton of value when I took the time to distill my thoughts into clear design goals. I did not do this early in the design process, I sort of just stuck to the notions and ideas in my head. Where I found the most value for design goals was later in the design process, where I found myself getting lost with random ideas and straying from the core idea. By taking the time to define my design goals, it helped me abandon all the random ideas and got me through the final push of the design hell process to finish the board's design.

[go above? ]So I really found design goals to be very instrumental in trimming down the infinite design decision space into much more manageable spaces to consider. For me, specifically, they helped to keep my overreaching ideas in check and help make decisions that were core to the original idea and not just fluff.

No Matrix.
The board would not have an LED matrix or any inkling of typical rectolinear LED placement. This first design goal is different from the later design goals--it was always set in stone, I was always keenly aware of the goal, and it came before everything else. It was my very next thought after I decided to think of a design. Why? It would be a great forcing function to funnel my thoughts into coming up with my own unique take on a hardware badge. Maker designs with LED matrices are a dime a dozen. The SuperCon Badge 2022 was a big inspiration here (despite the fact that it also has a matrix), it's a wonderful board where the LEDs aren't just pixels in a larger display but represent various state of the mocked 4-bit CPU.

Aesthetics > Gameplay
The consequence of the first design meant that the board's track and platform layout would be fixed. This would limit the overall potential for gameplay variablity and depth. So the decision to prioritize form over function came naturally. In other words, I consciously chose to value the board’s unique look/aesthetic appeal over making any changes to bolster the potential gameplay depth (like an LED matrix!). Despite this sort of "fixed board state" limitation, I still have good memories enjoying the Tiger handheld games as a kid, which were argulably even more limited with static LCD display graphics. So I was pretty confident that I could make an enjoyable game despite the limitations! Without this very strong attachment to this design choice, I would have definitely struggled with the fact that the track is fixed and really limits the gameplay variability. Each time however, I just reminded myself of this design goal!

Learning and Experience > Productization
While I mentioned earlier that I had always wanted to design my own hardware badge, in that similar vein I've always been really interested in designing my own products and selling them as a hobby, like many on Tindie, Etsy, Ebay, etc. already do so. However, I am also very aware of my own nature of trying to do too much at once and I wanted to give myself the space to not get too ahead of myself turning this project into a product before I had enough experience. So while this project itself isn't necessarily ~not~ a step in that direction--to sell my own designs as hobby--I found it extremely helpful to prioritize the learning experience over the productization, because if I didn't, my focus would have been divided.



===

### Choosing the Hardware

I leaned heavily into my recent experience with my smaller projects trying out different ideas on dev boards and Stemma QT/Qwiic boards with various types of LED matrices and arrangements. 
As a result of my experience with those projects, I was pretty adamant early on that I did not want to use Neopixels. I found them really finnicky to work with at dimmer levels and they draw more baseline current than standard LEDs. They would be more expensive and harder to repair by hand. But most importantly, when it comes to artificial light, I am all in on warm colors.

IS31 + LEDs: These ended up being a straightforward design choice for me. I had a good experience working with it in the Adafruit LED backpack, where I found it very easy to do smooth animations with full individial PWM control for each LED. I considered similar chips like the IS41, but thought that 144 LEDs was a good number. Red and yellow were the natural choices for two basic warm colors with maximum contrast.

Kingbright 7 Segments + AS1115: I chose the Kingbrights because they are smol and fit well in the board aesthetic of "components are buildings". There's no specific design decision with the AS1115, I just found a IC driver for seven segments that communicated over I2C and had an inline package that fit well.

ATMega32u4: I chose this primarily because of it's an AVR chip and it has built in USB. I had actually never used this chip before, but have experience with other AVR chips, so I saw the built in USB as an extra option for future ideas or debugging. But it's pretty expensive for what it is, ~$5/chip, so I definitely wouldn't have chosen this without the learning design goal. The USB feature would be especially useful when I gave boards to friends and family, as I could craft a simple script they could use to reprogram the boards, and I had no interest in adding a USB-to-serial chip.

Power Circuitry: Mostly chosen after I reviewed many schematics of other development boards and projects. I did some fairly hand-wavy math for my expected maximum current draw for my board, and made sure the LDO regulator could source well above that.

Buzzer: This same buzzer comes with the ACK1 coding kit hardware. The biggest factor was mostly how great it fit in the empty space on the board as a "building". It's a relatively large piezo buzzer and can be driven directly with an IC pic.

### Layout

Layout was definitely where the bulk of my time was spent. I first started by roughly laying out the outer loop of the track and placing the edge cuts around the outer loop, using Kicad's ray tracer to help me get a good idea of the size and spacing of the LEDs. I also held up a lot of stuff to my monitor to get a feel for holding it. From there, using my notebook sketch as an initial guide, I started to lay down the inner tracks and forks. Once I had a rough track, I had plently leftover LEDs for platforms that I placed along the track. I then tried to balance a few parameters for the track layout, drawing from some of my experience with my crazy Lego tracks:
- Length of track sections between forks/crosses
- Direction of forks along a given track
- Platform count in each track section

Once the track layout was done, I placed the rest of the components: ATMega near the bottom to be close to power and USB connector, IS31 in the middle to minimize trace length to LEDs, and the seven segments at the top with the AS1115 for the game display components. A button next to each fork and cross, and four control buttons at the bottom. I put the batteries holders on the back of the board, at the bottom. The batteries would both help hands to grip the board and also put the weight of the board in the hands for comfort.

Before I started routing traces, I was basically certain I was going to need a 4 layer board given the charlieplexing nature of the LED driver, so I started by defined the two inner layers as a power and ground fill. This effecively creates a capacitor out of the the two inner layers, which in theory would contribute to the voltage stability of the power supply along with the ceramic capacitors in the power circuitry (Future note to estimate the capacitance given the board area, copper thickness, and layer separation!). The IS31 operates the 144 LEDs as two separate 9x8 matrices, so I made sure to segment the LEDs into those groups on the board. However, after I finished the initial routing for the upper LEDs, I was not really happy with the traces, so I ended up doing some puzzle time, shifting around the LEDs to group common connections and try to minimize and balance the trace length of each line.

I tried an autorouter a few times midprocess, but ultimately I wanted to lay all the trace myself.

    § I should have done a little more planning before starting trace layout in regards to the placement of the LEDs to minim
    § KiCad annoyance: I tried to figure out the best way to work around a design rule issue where I would get errors for unconnected pads
○ Kicad:
    § Kicad footprint pads with implicit connections



### Order Prep
Before this project, I had only done one other project with PCBs of my own design, my [Stemmett project](https://github.com/nonik0/Light-Rail) so I had limited experience with the various options for PCB manufacturing/assembly. However, PCBWay had reached out to early me asking if to collaborate. Since I had not tried their services before, it made the whole process of choosing a PCB manufacturer very simple for me! Overall, I can say I have no issues that are really worth mentioning and only good things to say about my experience in retrospect.

For getting started with the ordering process, PCBWay has existing resources for KiCad that bootstrap this instantly. Just by installing PCBWay's plugin from KiCad's plugin manager, I could click a menu link that would directly open a browser to PCBWay's ordering page with my Gerbers already uploaded. This definitely saved a lot of time and clicks over my design iterations!

In addition to manufacturing the PCBs, I also made the decision early in the project timeline to assemble the boards. It let me worry less during the design process about the complexity or difficulty of hand assembling the boards and focus most on the aesthetics. It would also give me valuable experience for going through that process for the future. While I don’t have anything for comparison, my experience with PCBWay for the assembly was overall very smooth and painless!

The bulk of the assembly order preparation process was creating the Bill of Materials (BoM). I did this process manually using PCBWay's provided [BoM sample](http://www.pcbway.com/img/images/pcbway/Sample_BOM_PCBWay.xlsx) as a useful template. It was definitely valuable experience to have to suffer through a bit of datasheet hell, making sure the manufacturer part was the specific one, and learning the myriad of package types (and the different names and variations of the same ones!). I ended up relying heavily on copying and pasting the component data from Digikey's product pages, which I found much more consistent than the data on Mouser's product pages, especially the product descriptions.

 ### Ordering Process With PCBWay
Following my BoM completion, I could submit the order. Unfortunately, I realized right after I had submitted that I included the wrong version of the BoM. However, I was very pleased when I was almost immediately able to chat with an ostensibly human customer service through a chat window on my order page, where they were able to answer my questions--mainly that I could just email my assigned rep with the updates I needed.

Following order submission and prompt BoM correction, over the next few days I worked out the kinks in my layout with a few back and forths with the review team. The primary issue was that the minimum solder mask bridge width (i.e. the narrowest the soldermask can be) of the black solder mask I had chosen was slightly larger/less tolerant than the green, so I had to amend my design. The issue stemmed from a couple of the ICs on the board where the gap between the pins was too narrow for the solder mask. So the fix was to just not have soldermask in between the pins. While the lack of solderpaste initially concerned me due to increased risk of bridging, etc., after some reading I got the notion this was not uncommon, and also I was not planning to hand solder the chips so the risk was less. So in the end I was able to make the relatively simple changes to my layout to remove the soldermask between pins of two ICs, but the reason the process ended up being "a few back and forths" was due to my failure to fix the issue in one attempt. So, I want to detail a bit why it took me a few times. For one, primarily, I struggled with the Kicad settings to fix this. In fact, I still don't know how to fix this "in Kicad" (as I just tried again as I wrote this).

[TODO: solder mask picture]

With my first go at a fix, I found the "Solder mask minimum web width" in the solder mask/paste settings in Kicad layout editor. The mousey text for the setting reads "Min. dist between two pad areas. Two pads nearer this area will be merged during plotting". This sounds exactly like needed to accomodate the minimum solder mask bridge requirement. Solder mask bridge, solder mask web width--same thing. The default value was 0, so I set it to PCBWay's requirement (0.22mm). The change seemed so simple I literally didn't even check if changing the settting did anything. It didn't. The second go around I spent long enough trying to figure out what I needed to do to just get Kicad to fill in the damn gaps I evenutally just gave up and manually drew solder mask exclusion zones around the pads, which took about two minutes. So I sent things back to PCBWay, before I had to do one final revision since I had completley missed adding the solder mask adjustments to the other IC with the bridge issue.

The assembly process with PCBWay overall was much smoother than I anticipated. This was my first time getting anything assembled in a relatively "turnkey" way, so I was preparing myself to confront and deal with many hiccups and setbacks with the eventually goal of a functional prototype and valuable firsthand experience with "the process". So the fact I have a wholly working prototype is a great feeling and a good reflection at the relatively easy access PCBWay offers for these services. Overall, I feel like once I have a quality BoM for the projeect thn

The process started during the order review process where I was PCBWay gave me an updated BoM with quotes for the parts, as I had chosen for PCBWay to supply the parts. When I reviewed the first quote for the BoM I was surprised in a couple different ways. I was pleasantly surprised that the quotes PCBWay had given me for the components were, by and large, cheaper or at parity if I were to buy all the same parts from Mouser or Digikey. But I was also surprised at total cost of components per board! It really adds up, and it started giving me a flurry of ideas for how to get the cost per board down, but ultimately that was energy best spent much later since this process was about learning the whole process and not about the final cost optimizations for a final product whose cost I need to bring down as much as I can.

PARAGRAPH BELOW
- Was delayed a couple weeks. I missed a few mails where they were asking for answers to engineering questions, but they never actually sent them. I was checking daily on the website, where there is a clear tab for engineering questions, and there was no indication there.
- I would say this is my biggest point of feedback for PCBWay if they are reading this! To make sure engineering questions show up on order page as well if being sent in email.

So in between ordering and getting the boards I had to do another fairly simple questions and answers process for the assembly. They needed a few extra things, such as an extra image to help with the LED orientation and clarificaitons about solder mask for component I had marked DNA in the BoM (it didn't need any). A couple of issues came up during assembly that were workable. I made the mistake of swapping out the reset switch in the schematic and the BoM,but not updating the footprint in the layout. So the reset switch in the BoM's footprint didn't match my boards that were already manufactured. I was able to easily work around this by updating my BoM with a switch that matched the one I had, and PCBWay simpled sourced this other switch and the end result was only a day of delay! Ironically however, was another issue with the footprints of all the other buttons on the board, but at least this time it was something wholly unforseeable. The issue was the actual switches did not match the datasheet, which PCBWay also had noted. Luckily however, the discrepancy was only with the ground pins of the buttons so they could still be soldered and functional.

[TODO: switch and datasheet photo]

After about a week after getting the initial assemly questions sort, I got an email with annotaed pictures of an assembled board and questions asking if the orientations of the LEDs and the seven segments were correct. The LEDs looked correct, but the seven segmented I noted were upside down. The next day, I got a picture of a fixed board and I acknowledged as such. Four day later, my order was shipped! When the boards arrived, they came nearly perfect! There was only one assembly mistake where component C1 was missing on all the boards. Coincidenally, C1 (and C2) ended up being unecessary!


### Design Issues

#### Clock Source
I didn't choose an actual clock source till near the end where I changed the clock source component near the end and chose a CMOS oscillator, but my mistake was only looking at the pin mapping table in the datasheet and not the rest of it! That mistake caused the ATMegas to be non-functional and my boards didn't work at first. Luckily, and actually giving a bit of credit to myself for the many hours of reviewing things over and over (except the XO I gues), I was able to completely fix the issue!
-  I learned the differences between passive XTAL (crystal) and active XO (oscillator), as well as the functional differences for the Atmega fuse settings. When fuses are set for crystal oscillators, XTAL2 is part of the overall oscillator circuit, and is NOT stable. When I swapped the crystal oscillator to an active oscillator in my design, I just left XTAL2 connected to the "tri-state" pin for the XO. My brain didn't make the mental leap from "tri-state to "input" to "enable". When I re-read the datasheet it was clear. My first fix was a glob of solder between pin 3 and Vcc pin of the XO to keep the XO enabled, and I could finally talk to the ATMega!
- I later refined this approach by crafting a pogo pin jig to short the same two pins, then with the jig held in place, I could set the fuses to an external oscillator (not crystal!) with a programmer. With the new fuse settings for an exrternal clock, XTAL2 is not used and is an extra GPIO. So now even with the GPIO unconfigured (e.g. tri-state/high impedance), the external oscillator will be to function, as floating is a valid enable state for the XO's enable/tri-state pin.

Plugged in and was able to see the DFU in Windows Device Manager!

#### USB/USB Bootloader issue
Once I saw the DFU bootloader show up over USB, I wanted to try out the Arduino "Caterina" based bootloader. This is the Arduino bootloader you would typically find on Arduino boards with AVR boards like ATMega32u4 with built-in USB to serial, like Arduino Leonardo, Adafruit Feather 32u4, or Sparkfun Pro Micro. Unfortunately, that's about all I have to report as any varitation of the Caterina bootloader would just report as a dysfunctional USB device. So at the time I was feeling dejected and confused how the stock DFU bootloader was showing up but no bootloader I was trying were. Eventually, I wanted to revert to the stock DFU bootloader, to both sanity check that the boards would still show up over USB and also actually try DFU'ing with the DFU bootloader. For some reason, I had to find the original DFU bootloader hex from somewhere on Github than from Microchip...anything. So luckily once I found the binary and flashed it to the board, the boards once again showed up and I was able to use dfu-programmer to flash a hex! MY current theory is that the DFU bootloader works as its using the slower USB standard and the Caterina one is using a fast bus clock and it hitting some issue in my board's trace layout. 

### Conclusion and Next Steps

Overall, I am very satisfied with the turnout of this project. I learned a ton and got a lot of the experience I was seeking to have, especially during the investigations of for fixes/workarounds for the various issues that came up. I am happy and thankful that all of the issues that did come up were not insurmountable. While I do acknowledge some factor of luck, I did spend a lot of time going over my schematics and layout over and over and not rushing the process, letting my mind rest a lot in between reviewing everything. The mistakes I did make really were just reflections of the gaps in that review process, where I read the datasheets for the driver ICs over and over, yet I never read the full two page datasheet for the oscillator! Still, on the other hand, I could have read the datasheet countless times for the buttons I used, but couldn't have predicted the actual buttons not being to spec. So it just goes to show how you can never really plan or predict all the things that could happen, so plan for and expect it. But that fact does not allay your responsibility for due diligence in your own review process. Learning to focus your energies most on the things you can control is a widely applicable skill!


11/15: email bom update, solder mask birdge
11/16: order failed to review
11/17: order failed to review, then OK. ACK BOM quote
11/18: missed and respond to q
11/19: quotes asks
11/21: order
11/29: s13/back of board/led orientation question
12/8: eng q email
12/16: enq q email
12/19: eng q email, i repsod and get question (SW13 change, ground pins don't match)
12/20: i answer and ask pics
12/21: pics of board
12/30: pics of assembly, need to adjust 7 seg
1/1: updates pics are good
1/4 ships
