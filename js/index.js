/* global moment */

// Rust 1.5.0 was released on 2015-12-10
const EPOCH_DATE = moment.utc('2015-12-10')
const EPOCH_RELEASE = 5
const DATE_FORMAT = 'MMMM Do YYYY'
const newReleases = Math.floor(moment.utc().diff(EPOCH_DATE, 'weeks') / 6)

function addRelease (kind, incr, toolsWeek) {
  const releaseNumber = EPOCH_RELEASE + newReleases + incr
  const displayVersion = `1.${releaseNumber}`
  const releaseDate = EPOCH_DATE.clone().add((newReleases + incr) * 6, 'weeks')

  document.querySelector(`#${kind}-version`).textContent = displayVersion
  document.querySelector(`#${kind}-release-date`).textContent = `${releaseDate.format(DATE_FORMAT)} UTC`

  if (toolsWeek) {
    const noBreakagesTo = releaseDate.clone().day(2)
    const noBreakagesFrom = noBreakagesTo.clone().subtract(6, 'days')
    const toDate = noBreakagesTo.format(DATE_FORMAT)
    const fromDate = noBreakagesFrom.format(DATE_FORMAT)

    document.querySelector(`#${kind}-cycle`).textContent = displayVersion
    document.querySelector(`#${kind}-timespan`).textContent = `${fromDate} → ${toDate}`
  }
}

if (document.querySelector('#current-release-versions')) {
  addRelease('stable', 0, false)
  addRelease('beta', 1, true)
  addRelease('nightly', 2, true)
}
