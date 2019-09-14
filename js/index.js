// Rust 1.5.0 was released on 2015-12-10
const epochDate = moment.utc("2015-12-10");
const epochRelease = 5;
const dateFormat = "MMMM Do YYYY";

const newReleases = Math.floor(moment.utc().diff(epochDate, "weeks") / 6);

const addRelease = (kind, incr, tools_week) => {
    const releaseNumber = epochRelease + newReleases + incr;
    const displayVersion = `1.${releaseNumber}`
    const releaseDate = epochDate.clone().add((newReleases + incr) * 6, "weeks");

    document.querySelector(`#${kind}-version`).textContent = displayVersion;
    document.querySelector(`#${kind}-release-date`).textContent = releaseDate.format(dateFormat);

    if (tools_week === true) {
        const noBreakagesTo = releaseDate.clone().day(2);
        const noBreakagesFrom = noBreakagesTo.clone().subtract(6, 'day');
        const toDate = noBreakagesTo.format(dateFormat);
        const fromDate = noBreakagesFrom.format(dateFormat);

        document.querySelector(`#${kind}-cycle`).textContent = displayVersion;
        document.querySelector(`#${kind}-timespan`).textContent = `${fromDate} → ${toDate}`;
    }
};

addRelease("stable", 0, false);
addRelease("beta", 1, true);
addRelease("nightly", 2, true);
