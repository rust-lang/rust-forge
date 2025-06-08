/* global moment */

// Rust 1.5.0 was released on 2015-12-10
const EPOCH_DATE = moment.utc('2015-12-10')
const EPOCH_RELEASE = 5
const DATE_FORMAT = 'MMMM DD YYYY'
const newReleases = Math.floor(moment.utc().diff(EPOCH_DATE, 'weeks') / 6)

function addRelease (kind, incr, toolsWeek) {
  const releaseNumber = EPOCH_RELEASE + newReleases + incr;
  const displayVersion = `1.${releaseNumber}`;
  const releaseDate = EPOCH_DATE.clone().add((newReleases + incr) * 6, 'weeks');
  // We branch a little over 6 weeks before the release.
  const branchDate = EPOCH_DATE.clone().add((newReleases + incr - 1) * 6, 'weeks').subtract(6, 'days');

  document.querySelector(`#${kind}-version`).textContent = displayVersion
  document.querySelector(`#${kind}-release-date`).textContent = `${releaseDate.format(DATE_FORMAT)} UTC`
  document.querySelector(`#${kind}-branch-date`).textContent = `${branchDate.format(DATE_FORMAT)} UTC`

  if (toolsWeek) {
    const noBreakagesTo = releaseDate.clone().subtract(6, 'weeks').day(2)
    const noBreakagesFrom = noBreakagesTo.clone().subtract(6, 'days')
    const toDate = noBreakagesTo.format(DATE_FORMAT)
    const fromDate = noBreakagesFrom.format(DATE_FORMAT)

    document.querySelector(`#${kind}-cycle`).textContent = displayVersion
    document.querySelector(`#${kind}-timespan`).textContent = `${fromDate} → ${toDate}`
  }
}

if (document.querySelector('#current-release-versions')) {
  addRelease('stable', 0, false)
  addRelease('beta', 1, false)
  addRelease('nightly', 2, true)
  addRelease('next', 3, true)
}
