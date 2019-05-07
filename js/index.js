// Rust 1.5.0 was released on 2015-12-10
var epochDate = moment.utc("2015-12-10");
var epochRelease = 5;

var newReleases = Math.floor(moment.utc().diff(epochDate, "weeks") / 6);

function addRelease(kind, incr) {
    var releaseNumber = epochRelease + newReleases + incr;
    var releaseDate = epochDate.clone().add((newReleases + incr) * 6, "weeks");

    var out = "";
    out += '<div class="release">';
    out += '<div class="release-kind">Current ' + kind + '</div>';
    out += '<div class="release-number">1.' + releaseNumber + '</div>';
    out += '<div class="release-date">' + releaseDate.format("MMMM Do YYYY") + '</div>';
    out += '</div>';
    document.querySelector(".releases").innerHTML += out;
}

addRelease("stable", 0);
addRelease("beta", 1);
addRelease("nightly", 2);
