async function fetchAlbums() {
  try {
    const response = await fetch('/api/albums');
    const albums = await response.json();
    displayAlbums(albums);
  } catch (error) {
    console.error('Error fetching albums:', error);
  }
}

function displayAlbums(albums) {
  const tbody = document.getElementById('albums');
  tbody.innerHTML = '';

  albums.forEach(album => {
    const tr = document.createElement('tr');

    const tdArtwork = document.createElement('td');
    const img = document.createElement('img');
    img.className = 'album-artwork';
    img.src = album.artwork_url || 'https://placehold.co/48';
    img.alt = `${album.album} by ${album.artist}`;
    tdArtwork.appendChild(img);
    tr.appendChild(tdArtwork);

    const tdTitle = document.createElement('td');
    tdTitle.className = 'album-title';
    tdTitle.textContent = album.album;
    tr.appendChild(tdTitle);

    const tdArtist = document.createElement('td');
    tdArtist.className = 'album-artist';
    tdArtist.textContent = album.artist;
    tr.appendChild(tdArtist);

    const tdFormat = document.createElement('td');
    tdFormat.className = 'album-details';
    tdFormat.textContent = album.format.toUpperCase();
    tr.appendChild(tdFormat);

    const tdDate = document.createElement('td');
    tdDate.className = 'album-details';
    tdDate.textContent = (album.release_date || '').slice(0, 10);
    tr.appendChild(tdDate);

    tbody.appendChild(tr);
  });
}

document.addEventListener('DOMContentLoaded', fetchAlbums); 