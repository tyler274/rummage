/* Custom JavaScript for Rummage Documentation */

// Enhancement for status indicators
document.addEventListener('DOMContentLoaded', () => {
    // Add status indicator tooltips
    const statusElements = document.querySelectorAll('.status-indicator');
    statusElements.forEach(element => {
        if (element.classList.contains('complete')) {
            element.title = 'Implemented and tested';
        } else if (element.classList.contains('in-progress')) {
            element.title = 'In progress';
        } else if (element.classList.contains('planned')) {
            element.title = 'Planned but not yet implemented';
        }
    });

    // Implement tabs for code examples
    const tabContainers = document.querySelectorAll('.tabs');
    tabContainers.forEach(container => {
        const labels = container.querySelectorAll('.tab-label');
        const contents = container.querySelectorAll('.tab-content');

        labels.forEach((label, index) => {
            label.addEventListener('click', () => {
                // Remove active class from all labels and contents
                labels.forEach(l => l.classList.remove('active'));
                contents.forEach(c => c.classList.remove('active'));

                // Add active class to clicked label and corresponding content
                label.classList.add('active');
                contents[index].classList.add('active');
            });
        });

        // Activate first tab by default
        if (labels.length > 0) {
            labels[0].click();
        }
    });

    // Add copy button to code blocks
    document.querySelectorAll('pre').forEach(block => {
        if (block.querySelector('code')) {
            const button = document.createElement('button');
            button.className = 'copy-button';
            button.textContent = 'Copy';

            button.addEventListener('click', () => {
                const code = block.querySelector('code').textContent;
                navigator.clipboard.writeText(code).then(() => {
                    button.textContent = 'Copied!';
                    setTimeout(() => {
                        button.textContent = 'Copy';
                    }, 2000);
                });
            });

            block.style.position = 'relative';
            button.style.position = 'absolute';
            button.style.top = '5px';
            button.style.right = '5px';
            button.style.padding = '3px 8px';
            button.style.border = 'none';
            button.style.borderRadius = '3px';
            button.style.backgroundColor = '#4a5568';
            button.style.color = 'white';
            button.style.cursor = 'pointer';

            block.appendChild(button);
        }
    });
});

// Enhance navigation on small screens
window.addEventListener('resize', () => {
    const sidebar = document.querySelector('.sidebar');
    if (window.innerWidth < 768 && sidebar) {
        sidebar.classList.add('mobile');
    } else if (sidebar) {
        sidebar.classList.remove('mobile');
    }
});

// Initialize on load
window.dispatchEvent(new Event('resize')); 