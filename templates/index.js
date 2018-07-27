window.onload = function() {
    // var form = document.getElementById('message-form');
    var outputField = document.getElementById('game-output');
    var socketStatus = document.getElementById('status');
    // var closeBtn = document.getElementById('close');

    var socket = new WebSocket('ws://echo.websocket.org');
    socket.onopen = function(event) {
        socketStatus.innerHTML = 'Connected to: ' + event.currentTarget.url;
        socketStatus.className = 'open';
    };
    socket.onclose = function(event) {
        socketStatus.innerHTML = 'Disconnected from WebSocket.';
        socketStatus.className = 'closed';
    };
    socket.onerror = function(error) {
        console.log('WebSocket Error: ' + error);
    };
    socket.onmessage = function(event) {
        outputField.innerHTML = event.data;
    };
};
