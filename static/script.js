const messagesDiv = document.getElementById('messages');
const messageInput = document.getElementById('messageInput');
const sendButton = document.getElementById('sendButton');

async function loadChatHistory() {
    try {
        const response = await fetch('/api/chat/history');
        if (!response.ok) throw new Error('Fehler beim Laden der Historie');
        
        const messages = await response.json();
        messagesDiv.innerHTML = '';
        messages.forEach(msg => displayMessage(msg));
        scrollToBottom();
    } catch (error) {
        console.error('Fehler beim Laden:', error);
    }
}

function displayMessage(msg) {
    const msgEl = document.createElement('div');
    msgEl.className = `message ${msg.role}`;
    
    const contentDiv = document.createElement('div');
    contentDiv.className = 'message-content';
    
    // Für Assistant-Nachrichten: HTML rendern (Markdown wurde konvertiert)
    // Für User-Nachrichten: Text escapen
    if (msg.role === 'assistant') {
        contentDiv.innerHTML = msg.content;
    } else {
        contentDiv.textContent = msg.content;
    }
    
    msgEl.appendChild(contentDiv);
    messagesDiv.appendChild(msgEl);
}

function escapeHtml(text) {
    const map = {
        '&': '&amp;',
        '<': '&lt;',
        '>': '&gt;',
        '"': '&quot;',
        "'": '&#039;'
    };
    return text.replace(/[&<>"']/g, m => map[m]);
}

function scrollToBottom() {
    setTimeout(() => {
        messagesDiv.scrollTop = messagesDiv.scrollHeight;
    }, 0);
}

async function sendMessage() {
    const content = messageInput.value.trim();
    if (!content) return;

    messageInput.value = '';
    sendButton.disabled = true;

    // User message
    displayMessage({ role: 'user', content });

    // Loading indicator
    const loadingEl = document.createElement('div');
    loadingEl.className = 'message assistant';
    loadingEl.innerHTML = '<div class="message-content"><div class="loading"><span></span><span></span><span></span></div></div>';
    messagesDiv.appendChild(loadingEl);
    scrollToBottom();

    try {
        const response = await fetch('/api/chat/send', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ message: content })
        });

        if (!response.ok) throw new Error('Server-Fehler');

        const data = await response.json();
        loadingEl.remove();
        displayMessage({ role: 'assistant', content: data.message });
    } catch (error) {
        loadingEl.remove();
        displayMessage({
            role: 'assistant',
            content: '❌ Fehler bei der Kommunikation mit dem Server. Bitte versuche es später erneut.'
        });
        console.error('Fehler:', error);
    } finally {
        sendButton.disabled = false;
        messageInput.focus();
        scrollToBottom();
    }
}

messageInput.addEventListener('keypress', (e) => {
    if (e.key === 'Enter' && !e.shiftKey) {
        e.preventDefault();
        sendMessage();
    }
});

// Initial load
loadChatHistory();
