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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="intro.html">Introduction</a></li><li class="chapter-item expanded "><a href="navigation/intro.html"><strong aria-hidden="true">1.</strong> Navigation</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="navigation/topics.html"><strong aria-hidden="true">1.1.</strong> By topic</a></li><li class="chapter-item expanded "><div><strong aria-hidden="true">1.2.</strong> FAQs</div></li><li class="chapter-item expanded "><a href="navigation/index.html"><strong aria-hidden="true">1.3.</strong> Index</a></li></ol></li><li class="chapter-item expanded "><li class="part-title">Part 1: guide</li><li class="chapter-item expanded "><a href="part-guide/intro.html"><strong aria-hidden="true">2.</strong> Introduction</a></li><li class="chapter-item expanded "><a href="part-guide/concurrency.html"><strong aria-hidden="true">3.</strong> Concurrent programming</a></li><li class="chapter-item expanded "><a href="part-guide/async-await.html"><strong aria-hidden="true">4.</strong> Async and await</a></li><li class="chapter-item expanded "><a href="part-guide/more-async-await.html"><strong aria-hidden="true">5.</strong> More async/await topics</a></li><li class="chapter-item expanded "><a href="part-guide/io.html"><strong aria-hidden="true">6.</strong> IO and issues with blocking</a></li><li class="chapter-item expanded "><a href="part-guide/concurrency-primitives.html"><strong aria-hidden="true">7.</strong> Composing futures concurrently</a></li><li class="chapter-item expanded "><a href="part-guide/sync.html"><strong aria-hidden="true">8.</strong> Channels, locking, and synchronization</a></li><li class="chapter-item expanded "><a href="part-guide/tools.html"><strong aria-hidden="true">9.</strong> Tools for async programming</a></li><li class="chapter-item expanded "><a href="part-guide/dtors.html"><strong aria-hidden="true">10.</strong> Destruction and clean-up</a></li><li class="chapter-item expanded "><a href="part-guide/futures.html"><strong aria-hidden="true">11.</strong> Futures</a></li><li class="chapter-item expanded "><a href="part-guide/runtimes.html"><strong aria-hidden="true">12.</strong> Runtimes</a></li><li class="chapter-item expanded "><a href="part-guide/timers-signals.html"><strong aria-hidden="true">13.</strong> Timers and signal handling</a></li><li class="chapter-item expanded "><a href="part-guide/streams.html"><strong aria-hidden="true">14.</strong> Async iterators (streams)</a></li><li class="chapter-item expanded affix "><li class="part-title">Part 2: reference</li><li class="chapter-item expanded "><div><strong aria-hidden="true">15.</strong> Implementing futures and streams</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">16.</strong> Alternate runtimes</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">17.</strong> Implementing your own runtime</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">18.</strong> async in sync, sync in async</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">19.</strong> Async IO: readiness vs completion, and io_uring</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">20.</strong> Design patterns</div></li><li class="chapter-item expanded "><a href="part-reference/cancellation.html"><strong aria-hidden="true">21.</strong> Cancellation and cancellation safety</a></li><li class="chapter-item expanded "><div><strong aria-hidden="true">22.</strong> Starvation</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">23.</strong> Pinning</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">24.</strong> Async and FFI</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">25.</strong> Comparing async programming in Rust to other languages</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">26.</strong> The implementation of async/await in rustc</div></li><li class="chapter-item expanded "><a href="part-reference/structured.html"><strong aria-hidden="true">27.</strong> Structured concurrency</a></li><li class="chapter-item expanded affix "><li class="part-title">Old chapters</li><li class="chapter-item expanded "><a href="01_getting_started/01_chapter.html"><strong aria-hidden="true">28.</strong> Getting Started</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="01_getting_started/02_why_async.html"><strong aria-hidden="true">28.1.</strong> Why Async?</a></li><li class="chapter-item expanded "><a href="01_getting_started/03_state_of_async_rust.html"><strong aria-hidden="true">28.2.</strong> The State of Asynchronous Rust</a></li><li class="chapter-item expanded "><a href="01_getting_started/04_async_await_primer.html"><strong aria-hidden="true">28.3.</strong> async/.await Primer</a></li></ol></li><li class="chapter-item expanded "><a href="02_execution/01_chapter.html"><strong aria-hidden="true">29.</strong> Under the Hood: Executing Futures and Tasks</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="02_execution/02_future.html"><strong aria-hidden="true">29.1.</strong> The Future Trait</a></li><li class="chapter-item expanded "><a href="02_execution/03_wakeups.html"><strong aria-hidden="true">29.2.</strong> Task Wakeups with Waker</a></li><li class="chapter-item expanded "><a href="02_execution/04_executor.html"><strong aria-hidden="true">29.3.</strong> Applied: Build an Executor</a></li><li class="chapter-item expanded "><a href="02_execution/05_io.html"><strong aria-hidden="true">29.4.</strong> Executors and System IO</a></li></ol></li><li class="chapter-item expanded "><a href="03_async_await/01_chapter.html"><strong aria-hidden="true">30.</strong> async/await</a></li><li class="chapter-item expanded "><a href="04_pinning/01_chapter.html"><strong aria-hidden="true">31.</strong> Pinning</a></li><li class="chapter-item expanded "><a href="05_streams/01_chapter.html"><strong aria-hidden="true">32.</strong> Streams</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="05_streams/02_iteration_and_concurrency.html"><strong aria-hidden="true">32.1.</strong> Iteration and Concurrency</a></li></ol></li><li class="chapter-item expanded "><a href="06_multiple_futures/01_chapter.html"><strong aria-hidden="true">33.</strong> Executing Multiple Futures at a Time</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="06_multiple_futures/02_join.html"><strong aria-hidden="true">33.1.</strong> join!</a></li><li class="chapter-item expanded "><a href="06_multiple_futures/03_select.html"><strong aria-hidden="true">33.2.</strong> select!</a></li><li class="chapter-item expanded "><a href="06_multiple_futures/04_spawning.html"><strong aria-hidden="true">33.3.</strong> Spawning</a></li><li class="chapter-item expanded "><div><strong aria-hidden="true">33.4.</strong> TODO: Cancellation and Timeouts</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">33.5.</strong> TODO: FuturesUnordered</div></li></ol></li><li class="chapter-item expanded "><a href="07_workarounds/01_chapter.html"><strong aria-hidden="true">34.</strong> Workarounds to Know and Love</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="07_workarounds/03_send_approximation.html"><strong aria-hidden="true">34.1.</strong> Send Approximation</a></li><li class="chapter-item expanded "><a href="07_workarounds/04_recursion.html"><strong aria-hidden="true">34.2.</strong> Recursion</a></li><li class="chapter-item expanded "><a href="07_workarounds/05_async_in_traits.html"><strong aria-hidden="true">34.3.</strong> async in Traits</a></li></ol></li><li class="chapter-item expanded "><a href="08_ecosystem/00_chapter.html"><strong aria-hidden="true">35.</strong> The Async Ecosystem</a></li><li class="chapter-item expanded "><a href="09_example/00_intro.html"><strong aria-hidden="true">36.</strong> Final Project: HTTP Server</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="09_example/01_running_async_code.html"><strong aria-hidden="true">36.1.</strong> Running Asynchronous Code</a></li><li class="chapter-item expanded "><a href="09_example/02_handling_connections_concurrently.html"><strong aria-hidden="true">36.2.</strong> Handling Connections Concurrently</a></li><li class="chapter-item expanded "><a href="09_example/03_tests.html"><strong aria-hidden="true">36.3.</strong> Testing the Server</a></li></ol></li><li class="chapter-item expanded "><div><strong aria-hidden="true">37.</strong> TODO: I/O</div></li><li><ol class="section"><li class="chapter-item expanded "><div><strong aria-hidden="true">37.1.</strong> TODO: AsyncRead and AsyncWrite</div></li></ol></li><li class="chapter-item expanded "><div><strong aria-hidden="true">38.</strong> TODO: Asynchronous Design Patterns: Solutions and Suggestions</div></li><li><ol class="section"><li class="chapter-item expanded "><div><strong aria-hidden="true">38.1.</strong> TODO: Modeling Servers and the Request/Response Pattern</div></li><li class="chapter-item expanded "><div><strong aria-hidden="true">38.2.</strong> TODO: Managing Shared State</div></li></ol></li><li class="chapter-item expanded "><a href="12_appendix/01_translations.html"><strong aria-hidden="true">39.</strong> Appendix: Translations of the Book</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
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
