use crate::components::blog_post::BlogPostProps;
use chrono::NaiveDate;
use yew::prelude::*;

pub fn dont_mock_your_persistence_layer() -> BlogPostProps {
    let title = String::from("Don't mock your persistence layer");
    let published = NaiveDate::from_ymd_opt(2023, 6, 2).unwrap();
    let content = html! {
        <>
            <p> {
                "When you read blog posts or articles about how to write integration tests for your apps,
                you often come across the idea that the persistance layer should be mocked or stubbed out.
                It is often claimed that speed is of utmost importance and anything that can be done to
                reduce the test execution time should be prioritized. I've self made this mistake several
                times..."
            } </p>
            <p> {
                "Mocking a persistence layer can be fine if your data model is simple. However as soon as
                your persistance layer gets involved in your business logic you will be in trouble. Lets say
                that your app's data model is getting more and more complex. You're adding unique constraints and
                foreign key relations with cascading deletes. You configure your database to handle this for you
                because you don't want to write logic yourself that a database can probably handle more efficiently.
                But what about the tests? With a mocked database you will have some serious expectations setup
                in each test case if you want to test these things... If you did like me you might think that
                its better to roll your own mini in memory database test double with all logic required for
                the test cases to run. But down this road you're basically implementing a complete database
                instead of adding business value to your app. And whats even worse is that you're not testing
                your app in a production like environment. The logic configured by your database is part of your
                app and should therefore be a part of your app during tests."
            } </p>
            <q> {
                "Ok I get it. Use a real database or redis server during tests... But what about external API calls?"
            } </q>
            <p> {
                "It depends on the purpose of the test. If you're writing integration tests for a single app, you're
                most likely better of mocking these. Making real requests to other apps will always be flaky and
                there is a large risk of getting false negative tests for a number of reasons. Some tests known as
                end to end tests (e2e) are used to test a system of many microservices and how they interact.
                These tests almost always target real apps running in either a staging or production environment.
                These tests ofcourse don't mock anything so the test coverage for each single test is very large.
                You should not write too many of these tests however as real systems (as we all know) are flaky
                and you don't want to spend time debugging false negative tests."
            } </p>
        </>
    };

    BlogPostProps {
        title,
        published,
        content,
    }
}
