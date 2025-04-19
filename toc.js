// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="index.html">Overview</a></li><li class="chapter-item expanded "><a href="platforms/index.html">Platforms</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="platforms/twitter.html">Twitter</a></li><li class="chapter-item expanded "><a href="platforms/discord.html">Discord</a></li><li class="chapter-item expanded "><a href="platforms/email.html">Email</a></li><li class="chapter-item expanded "><a href="platforms/github.html">GitHub</a></li><li class="chapter-item expanded "><a href="platforms/zulip.html">Zulip</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="platforms/zulip/moderation.html">Moderation</a></li></ol></li><li class="chapter-item expanded "><a href="platforms/blogs.html">Blogs</a></li><li class="chapter-item expanded "><a href="platforms/calendars.html">Calendars</a></li></ol></li><li class="chapter-item expanded "><a href="triagebot/index.html">Triagebot</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="triagebot/agenda.html">Agenda Generator</a></li><li class="chapter-item expanded "><a href="triagebot/issue-assignment.html">Issue Assignment</a></li><li class="chapter-item expanded "><a href="triagebot/pr-assignment.html">PR Assignment</a></li><li class="chapter-item expanded "><a href="triagebot/pr-assignment-tracking.html">Tracking PR assignment</a></li><li class="chapter-item expanded "><a href="triagebot/autolabels.html">Autolabels</a></li><li class="chapter-item expanded "><a href="triagebot/canonicalize-issue-links.html">Canonicalize Issue Links</a></li><li class="chapter-item expanded "><a href="triagebot/close.html">Close</a></li><li class="chapter-item expanded "><a href="triagebot/doc-updates.html">Documentation Updates</a></li><li class="chapter-item expanded "><a href="triagebot/github-releases.html">GitHub Releases</a></li><li class="chapter-item expanded "><a href="triagebot/glacier.html">Glacier</a></li><li class="chapter-item expanded "><a href="triagebot/issue-links.html">Issue Links</a></li><li class="chapter-item expanded "><a href="triagebot/transfer.html">Issue Transfer</a></li><li class="chapter-item expanded "><a href="triagebot/labeling.html">Labeling</a></li><li class="chapter-item expanded "><a href="triagebot/major-changes.html">Major Changes</a></li><li class="chapter-item expanded "><a href="triagebot/mentions.html">Mentions</a></li><li class="chapter-item expanded "><a href="triagebot/merge-conflicts.html">Merge Conflicts</a></li><li class="chapter-item expanded "><a href="triagebot/no-merge.html">No Merge Policy</a></li><li class="chapter-item expanded "><a href="triagebot/no-mentions.html">No Mentions</a></li><li class="chapter-item expanded "><a href="triagebot/nominate.html">Nominate</a></li><li class="chapter-item expanded "><a href="triagebot/note.html">Note</a></li><li class="chapter-item expanded "><a href="triagebot/notifications.html">Notifications</a></li><li class="chapter-item expanded "><a href="triagebot/pinging.html">Pinging</a></li><li class="chapter-item expanded "><a href="triagebot/rendered-link.html">Rendered link</a></li><li class="chapter-item expanded "><a href="triagebot/requesting-prioritization.html">Requesting Prioritization</a></li><li class="chapter-item expanded "><a href="triagebot/review-submitted.html">Review Changes Requested</a></li><li class="chapter-item expanded "><a href="triagebot/review-requested.html">Review Requested</a></li><li class="chapter-item expanded "><a href="triagebot/rustc-commit-list.html">Rustc Commit Tracking</a></li><li class="chapter-item expanded "><a href="triagebot/shortcuts.html">Shortcuts</a></li><li class="chapter-item expanded "><a href="triagebot/triage-dashboard.html">Triagebot Dashboard</a></li><li class="chapter-item expanded "><a href="triagebot/zulip-meeting.html">Zulip Meeting Management</a></li><li class="chapter-item expanded "><a href="triagebot/zulip-notifications.html">Zulip Notifications</a></li><li class="chapter-item expanded "><a href="triagebot/bot-pull-requests.html">GitHub Actions created PR open/closer</a></li></ol></li><li class="chapter-item expanded "><a href="community/index.html">Community</a></li><li class="chapter-item expanded "><a href="compiler/index.html">Compiler</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="compiler/calendar.html">Calendar</a></li><li class="chapter-item expanded "><a href="compiler/cross-team-collaboration.html">Cross-team Collaboration</a></li><li class="chapter-item expanded "><a href="compiler/meetings.html">Meetings</a></li><li class="chapter-item expanded "><a href="compiler/membership.html">Membership</a></li><li class="chapter-item expanded "><a href="compiler/notification-groups.html">Notification groups</a></li><li class="chapter-item expanded "><a href="compiler/resources.html">Resources</a></li><li class="chapter-item expanded "><a href="compiler/reviews.html">Review Policy</a></li><li class="chapter-item expanded "><a href="compiler/proposals-and-stabilization.html">Proposals, Approval and Stabilization</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="compiler/proposals-and-stabilization/ecosystem-integration-tests.html">Adding ecosystem/integration test jobs/components to rust-lang/rust CI</a></li></ol></li><li class="chapter-item expanded "><a href="compiler/third-party-out-of-tree.html">Third-party and Out-of-tree Crates Policy</a></li><li class="chapter-item expanded "><a href="compiler/prioritization.html">Triage and Prioritization</a></li><li class="chapter-item expanded "><a href="compiler/working-areas.html">Working Areas</a></li><li class="chapter-item expanded "><a href="compiler/operations.html">Operations</a></li></ol></li><li class="chapter-item expanded "><a href="crates-io/index.html">crates.io</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="crates-io/crate-removal.html">Crate removal</a></li><li class="chapter-item expanded "><a href="crates-io/db-maintenance.html">Database maintenance</a></li></ol></li><li class="chapter-item expanded "><a href="docs-rs/index.html">docs.rs</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="docs-rs/add-dependencies.html">Adding dependencies to the build environment</a></li><li class="chapter-item expanded "><a href="docs-rs/self-hosting.html">Self-hosting a docs.rs instance</a></li><li class="chapter-item expanded "><a href="docs-rs/maintenance.html">Maintenance procedures</a></li></ol></li><li class="chapter-item expanded "><a href="governance/index.html">Governance</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="governance/council.html">Leadership Council</a></li><li class="chapter-item expanded "><a href="governance/moderation.html">Moderation</a></li><li class="chapter-item expanded "><a href="governance/project-groups.html">Project groups</a></li></ol></li><li class="chapter-item expanded "><a href="policies/index.html">Policies</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="policies/crate-ownership.html">Crate ownership policy</a></li></ol></li><li class="chapter-item expanded "><a href="infra/index.html">Infrastructure</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="infra/other-installation-methods.html">Other Installation Methods</a></li><li class="chapter-item expanded "><a href="infra/archive-stable-version-installers.html">Archive of Rust Stable Standalone Installers</a></li><li class="chapter-item expanded "><a href="infra/channel-layout.html">Release Channel Layout</a></li><li class="chapter-item expanded "><a href="infra/service-infrastructure.html">Service Infrastructure</a></li><li class="chapter-item expanded "><a href="infra/team-maintenance.html">Team Maintenance</a></li><li class="chapter-item expanded "><a href="infra/toolstate.html">The Toolstate System</a></li><li class="chapter-item expanded "><a href="infra/policies/index.html">Policies</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="infra/policies/broken-nightlies.html">Broken nightlies</a></li></ol></li><li class="chapter-item expanded "><a href="infra/guidelines/index.html">Guidelines</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="infra/guidelines/static-websites.html">Static websites</a></li></ol></li><li class="chapter-item expanded "><a href="infra/docs/index.html">Documentation</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="infra/docs/aws-access.html">AWS access for team members</a></li><li class="chapter-item expanded "><a href="infra/docs/aws-access-management.html">AWS access management</a></li><li class="chapter-item expanded "><a href="infra/docs/aws-regions.html">AWS regions</a></li><li class="chapter-item expanded "><a href="infra/docs/bastion.html">Bastion server</a></li><li class="chapter-item expanded "><a href="infra/docs/bors.html">Bors</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="infra/docs/bors/queue-resync.html">Fixing bors queue</a></li></ol></li><li class="chapter-item expanded "><a href="infra/docs/cdn.html">CDN</a></li><li class="chapter-item expanded "><a href="infra/docs/crater-agents.html">Crater agents</a></li><li class="chapter-item expanded "><a href="infra/docs/dev-desktop.html">Dev Desktops</a></li><li class="chapter-item expanded "><a href="infra/docs/dev-desktop-github-app.html">GitHub App for dev-desktops</a></li><li class="chapter-item expanded "><a href="infra/docs/discord-mods-bot.html">Discord moderation bot</a></li><li class="chapter-item expanded "><a href="infra/docs/dns.html">Domain names and DNS</a></li><li class="chapter-item expanded "><a href="infra/docs/docs-rs.html">docs.rs</a></li><li class="chapter-item expanded "><a href="infra/docs/ecs-services.html">ECS services management</a></li><li class="chapter-item expanded "><a href="infra/docs/monitoring.html">Monitoring</a></li><li class="chapter-item expanded "><a href="infra/docs/rust-bots.html">rust-bots server</a></li><li class="chapter-item expanded "><a href="infra/docs/rustc-ci.html">rust-lang/rust CI</a></li><li class="chapter-item expanded "><a href="infra/docs/sentry.html">Sentry</a></li></ol></li></ol></li><li class="chapter-item expanded "><a href="lang/index.html">Language</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="lang/rfc-merge-procedure.html">RFC Merge Procedure</a></li><li class="chapter-item expanded "><a href="lang/triage-meeting-procedure.html">Triage Meeting Procedure</a></li><li class="chapter-item expanded "><a href="lang/stabilization-procedure.html">Stabilization procedure</a></li></ol></li><li class="chapter-item expanded "><a href="libs/index.html">Libs</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="libs/maintaining-std.html">Maintaining the standard library</a></li></ol></li><li class="chapter-item expanded "><a href="release/index.html">Release</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="release/backporting.html">Backporting</a></li><li class="chapter-item expanded "><a href="release/release-notes.html">Preparing Release Notes</a></li><li class="chapter-item expanded "><a href="release/process.html">Release Process</a></li><li class="chapter-item expanded "><a href="release/rollups.html">Rollup Procedure</a></li><li class="chapter-item expanded "><a href="release/triage-procedure.html">Triage Procedure</a></li><li class="chapter-item expanded "><a href="release/issue-triaging.html">Issue Triaging</a></li><li class="chapter-item expanded "><a href="release/crater.html">Triaging Crater Runs</a></li></ol></li><li class="chapter-item expanded "><a href="editions/edition-releases.html">Edition Releases</a></li><li class="chapter-item expanded "><a href="archive/index.html">Archive</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="archive/fott.html">Friends of the Tree</a></li><li class="chapter-item expanded "><a href="archive/release-history.html">Release History</a></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
