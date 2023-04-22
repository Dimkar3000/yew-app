# Counter And Todo App in yew

A Sample project using [Yew](http://www.google.com) And [daisyUI]() to create the sample apps every body makes in order to learn a certain technology.

## Features

- **daisyUI**: A good looking css framework to make the app look good
- **multiple projects**: I seperated the proeject in multiple lib packages that handle seperate concern in order to improve build performance. This is recomended by the Yew wiki
- **routing**: The landing page has 2 cards to select which project you want to view.
- **local storage**: In order to avoid to have to build a server to I created a service project that simulates its but it stores everything in local storage.

## How to run
Eseentially you need to run `trunk serve` inside the `package/app` folder that contains the executable. In order to build the css you can run the tailwind script in the node module after a `npm install`
### tldr
```bash
# terminal 1
npm run tailwind
# terminal 2
cd packages/app
trunk serve
```