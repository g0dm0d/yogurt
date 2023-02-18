export async function playNotificationSound() {
    const sound = new Audio('./notification.mp3');
    sound.addEventListener('canplaythrough', () => {
        sound.currentTime = 0;
        sound.play();
    });
}