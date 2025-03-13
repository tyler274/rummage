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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item affix "><a href="index.html">Introduction</a></li><li class="chapter-item affix "><li class="part-title">Commander Rules</li><li class="chapter-item "><a href="commander/index.html"><strong aria-hidden="true">1.</strong> Commander Format</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="commander/overview/index.html"><strong aria-hidden="true">1.1.</strong> Overview</a></li><li class="chapter-item "><a href="commander/game_mechanics/index.html"><strong aria-hidden="true">1.2.</strong> Game Mechanics</a></li><li class="chapter-item "><a href="commander/player_mechanics/index.html"><strong aria-hidden="true">1.3.</strong> Player Mechanics</a></li><li class="chapter-item "><a href="commander/zones/index.html"><strong aria-hidden="true">1.4.</strong> Game Zones</a></li><li class="chapter-item "><a href="commander/turns_and_phases/index.html"><strong aria-hidden="true">1.5.</strong> Turns and Phases</a></li><li class="chapter-item "><a href="commander/stack_and_priority/index.html"><strong aria-hidden="true">1.6.</strong> Stack and Priority</a></li><li class="chapter-item "><a href="commander/combat/index.html"><strong aria-hidden="true">1.7.</strong> Combat</a></li><li class="chapter-item "><a href="commander/special_rules/index.html"><strong aria-hidden="true">1.8.</strong> Special Rules</a></li></ol></li><li class="chapter-item "><li class="part-title">Game UI</li><li class="chapter-item "><a href="game_gui/index.html"><strong aria-hidden="true">2.</strong> Game UI System</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="game_gui/overview.html"><strong aria-hidden="true">2.1.</strong> Overview</a></li><li class="chapter-item "><a href="game_gui/layout/index.html"><strong aria-hidden="true">2.2.</strong> Layout Components</a></li><li class="chapter-item "><a href="game_gui/table/index.html"><strong aria-hidden="true">2.3.</strong> Table View</a></li><li class="chapter-item "><a href="game_gui/playmat/index.html"><strong aria-hidden="true">2.4.</strong> Playmat Design</a></li><li class="chapter-item "><a href="game_gui/chat/index.html"><strong aria-hidden="true">2.5.</strong> Chat System</a></li><li class="chapter-item "><a href="game_gui/avatar/index.html"><strong aria-hidden="true">2.6.</strong> Avatar System</a></li><li class="chapter-item "><a href="game_gui/testing/index.html"><strong aria-hidden="true">2.7.</strong> Testing</a></li></ol></li><li class="chapter-item "><li class="part-title">Networking</li><li class="chapter-item "><a href="networking/index.html"><strong aria-hidden="true">3.</strong> Networking</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="networking/core/architecture_overview.html"><strong aria-hidden="true">3.1.</strong> Core Networking</a></li><li class="chapter-item "><a href="networking/lobby/index.html"><strong aria-hidden="true">3.2.</strong> Lobby System</a></li><li class="chapter-item "><a href="networking/gameplay/index.html"><strong aria-hidden="true">3.3.</strong> Gameplay Networking</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="networking/gameplay/state/index.html"><strong aria-hidden="true">3.3.1.</strong> State Management</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="networking/gameplay/state/rollback.html"><strong aria-hidden="true">3.3.1.1.</strong> Rollback System</a></li><li class="chapter-item "><a href="networking/gameplay/state/replicon_rollback.html"><strong aria-hidden="true">3.3.1.2.</strong> Replicon Integration</a></li></ol></li></ol></li><li class="chapter-item "><a href="networking/testing/index.html"><strong aria-hidden="true">3.4.</strong> Testing</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="networking/testing/overview.html"><strong aria-hidden="true">3.4.1.</strong> Overview</a></li><li class="chapter-item "><a href="networking/testing/rng_synchronization_tests.html"><strong aria-hidden="true">3.4.2.</strong> RNG Synchronization Tests</a></li><li class="chapter-item "><a href="networking/testing/replicon_rng_tests.html"><strong aria-hidden="true">3.4.3.</strong> Replicon RNG Tests</a></li></ol></li><li class="chapter-item "><a href="networking/security/index.html"><strong aria-hidden="true">3.5.</strong> Security</a></li></ol></li><li class="chapter-item "><li class="part-title">Reference</li><li class="chapter-item "><a href="api/index.html"><strong aria-hidden="true">4.</strong> API Reference</a></li><li class="chapter-item "><a href="mtg_rules/index.html"><strong aria-hidden="true">5.</strong> MTG Rules Reference</a></li><li class="chapter-item "><a href="CONTRIBUTING.html"><strong aria-hidden="true">6.</strong> Contribution Guidelines</a></li><li class="chapter-item "><a href="contributing/documentation.html"><strong aria-hidden="true">7.</strong> Documentation Guide</a></li></ol>';
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
