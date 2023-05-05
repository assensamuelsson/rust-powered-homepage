use yew::prelude::*;
use crate::components::blog_post::BlogPostProps;
use chrono::NaiveDate;

pub fn test_as_examples() -> BlogPostProps {
    let title = String::from("Treat your tests as code examples for your app");
    let published = NaiveDate::from_ymd_opt(2023, 5, 5).unwrap();
    let content = html! {
        <>
            <p> {
                "Picture yourself in this situation - You're trying to learn a new library or framework.
                You go to their documentation or README and read the installation and getting started parts pretty quickly.
                Pretty soon (if you're like me) you'll get the feeling that you want to get your hands dirty - SHOW ME THE CODE!
                You scroll to find some code examples and finally you'll get an understanding of how to use the dependency."
            } </p>
            <p> {
                "Now picture another situtation - A coworker on your team (or you in the future) is trying to figure out an app that you wrote.
                Perhaps they start out by reading the README or any kind of documentation that you've left behind.
                If the app is big you probably didn't document all the details of how the app works.
                Besides, keeping documentation up to date is difficult to get right.
                So what do they do? Likely they'll start reading the code - starting from the entry point and going on from there.
                Again if the app is big this can be difficult and how would the person know if he or she got all the details right?
                What if there is a better way to explain how your app works?
                By now you've probably figured out where this is going..."
            } </p>
            <p> {
                "If you have practiced TDD (which you should), you have functional tests for everything in your app.
                Every nitty gritty detail is there. What if these tests could be used as code examples for your app?"
            } </p>
            <q> {
                "Yeah right... The tests are a mess and they include so much boilerplate that they are not readable at all..."
            } </q>
            <p> {
                "If you treat your tests as a necessary evil to test what really matters, the production code, then yeah - they probably are a mess.
                However, have you considered that your tests might be more important than your production code?
                Noy only are they veryfing that your app is working as intended, they can also be a great source of documentation."
            } </p>
            <q> {
                "Ok... Show how should I write my tests then?"
            } </q>
            <p> {
                "If you're into TDD (you should) you are used to developing your apps in a feedback loop often referred to as the red-green-refactor cycle.
                You write a test that fails (red), then you write the production code to make the test pass (green) and then you refactor the code to make it better (refactor).
                Do not forget to refactor your tests as well!
                Imagine that your app is a really popular open source library and that your tests are the code examples.
                You wouldn't want your very popular code examples to look like a mess would you?
                No you wan't them the be short, easy to understand and to explain you app's functionality as good as possible."
            } </p>
            <p> {
                "Now that you consider your tests as code examples, you should use them as a source to find out how your app works.
                The next time you or your colleagues are wondering e.g. \"How do your app call this other microservice given this request?\", you should look in the tests.
                If you manage to do this well you'll find that your tests are a great source of documentation for your app.
                And one of the bests things is that they are always up to date!"
            } </p>
            </>
    };

    BlogPostProps {
        title,
        published,
        content,
    }
}