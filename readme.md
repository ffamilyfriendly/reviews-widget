# Top.gg reviews widget demo
Herein is the code for the top.gg review widget demo. The code is written in rust (blazingly fast🔥) with the [rocket.rs](https://rocket.rs/) framework. Initially I foolishly scraped the website to fetch reviews but then found the unofficial [https://top.gg/api/client/entities/\<entity\>/reviews](https://top.gg/api/client/entities/870715447136366662/reviews) endpoint and re-wrote the code to use that. API responses are cached for a faster experience.

## Tasks
|Task|Description|Status|
|---|---|---|
|iframe widget|a simple and easy to use iframe widget that can be implemented in a matter of minutes|done|
|PNG widget|the idea with the PNG widget is to allow people to embed a overview of their reviews in places where an iframe could not be used. For example on a github markdown file like this one|not started|
|ratelimits|Right now it'd be trivial to DOS my widget by requesting random entity IDs. This would probably overwhelm my poor server and would absolutely get me ratelimited by top.gg. This is not yet implemented as this project is just an example for the AMA|not started|
|response caching|Right now only API responses directly from top.gg are cached. I do also want to cache the rendered handlebar files as that would lower the response time and make my server happier|not started|

## Implementing
The whole idea of the widget is for it to be plug and play. All you really need to do is add the iframe and style it a bit so it fits the layout of the website. I do understand that my design is not the best and some devs might prefer to style the widget for themselves so I included a css query that allows devs to link their own styling. The classnames for all required elements is included at the top of [standard.css](https://github.com/ffamilyfriendly/reviews-widget/blob/main/static/standard.css).


### barebones
```html
<iframe id="topgg-widget" loading="lazy" style="border:0;" width="100%" src="https://widget.familyfriendly.xyz/embed/<bot_id>">
            your browser does not support iframes
</iframe>
```

### custom css
```html
<iframe id="topgg-widget" loading="lazy" style="border:0;" width="100%" src="https://widget.familyfriendly.xyz/embed/<bot_id>?css=https%3A%2F%2Fexample.com%2Fwidget.css">
            your browser does not support iframes
</iframe>
```

## Issues
we obviously __do not__ want developers to reward users of their entity for leaving a good review. This is already kind of handled by the [simple algo](https://github.com/ffamilyfriendly/reviews-widget/blob/main/src/fetcher/fetch.rs#L52C73-L52C73) but some further changes might be required. Possibly the image URI could be anonymised in a way that makes it hard for devs to just get the user id from that. All in all I dont think it'll be an issue

## Pros
* a CTA for users to add their own review could be added that drives users to the top.gg page. Basically free advertising and perhaps good for SEO (I've seen folks say that iframes are generally ignored by the google crawler but that is bro science)
* with enough adoption there might be enough traffic to the widget to justify placing an auctions slot there. This would be cash money 
* last but not least I believe this would be good for devs who want to incorporate a testimonials section on their bots website. I went to top.gg and manually hardcoded reviews into [my bots website](https://threadwatcher.xyz) which is what led me to this idea.