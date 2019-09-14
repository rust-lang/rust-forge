// Rust 1.5.0 was released on 2015-12-10
const epochDate = moment.utc("2015-12-10");
const epochRelease = 5;

const newReleases = Math.floor(moment.utc().diff(epochDate, "weeks") / 6);

const addRelease = (kind, incr, tools_week) => {
    let releaseNumber = epochRelease + newReleases + incr;
    let releaseDate = epochDate.clone().add((newReleases + incr) * 6, "weeks");

    let out = "";
    out += '<div class="release">';
    out += '<div class="release-kind">Current ' + kind + '</div>';
    out += '<div class="release-number">1.' + releaseNumber + '</div>';
    out += '<div class="release-date">' + releaseDate.format("MMMM Do YYYY") + '</div>';
    out += '</div>';
    document.querySelector(".releases").innerHTML += out;

    if (tools_week === true) {
        let noBreakagesTo = releaseDate.clone().day(2);
        let noBreakagesFrom = noBreakagesTo.clone().subtract(1, 'week');

        let out = "";
        out += '<div class="tool-no-breakage">';
        out += '<div class="tool-no-breakage-cycle">1.' + releaseNumber + ' cycle</div>';
        out += '<div class="tool-no-breakage-dates">' + noBreakagesFrom.format("MMMM Do YYYY") + ' &rarr; ' + noBreakagesTo.format("MMMM Do YYYY") + '</div>';
        out += '</div>';
        document.querySelector(".tools-no-breakages").innerHTML += out;
    }
};

addRelease("stable", 0, false);
addRelease("beta", 1, true);
addRelease("nightly", 2, true);
