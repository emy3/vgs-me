// app.tsx

import React, { useEffect, useState } from 'react';

interface Message {
    text: string;
    vgs: string;
}

interface Messages {
    attack: Message[];
    defend: Message[];
}

const App: React.FC = () => {
    const [messages, setMessages] = useState<Messages | null>(null);

    useEffect(() => {
        const fetchData = async () => {
            try {
                const response = await fetch('/api/get_messages'); // Update the fetch URL to match your backend endpoint
                if (!response.ok) {
                    throw new Error('Failed to fetch data');
                }
                const data = await response.json();
                setMessages(data);
            } catch (error) {
                console.error('Error fetching data:', error);
            }
        };

        fetchData();
    }, []);

    return (
        <div>
            <h1>Messages</h1>
            {messages && (
                <div>
                    <h2>Attack Messages</h2>
                    <ul>
                        {messages.attack.map((message, index) => (
                            <li key={index}>{message.text}</li>
                        ))}
                    </ul>
                    <h2>Defend Messages</h2>
                    <ul>
                        {messages.defend.map((message, index) => (
                            <li key={index}>{message.text}</li>
                        ))}
                    </ul>
                </div>
            )}
        </div>
    );
};

export default App;
