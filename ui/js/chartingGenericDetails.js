const urlQueryString = window.location.search;
const invoke = window.__TAURI__.invoke;
const urlParams = new URLSearchParams(urlQueryString);

async function redeem_ticket(ticket) {
  return invoke('redeem_ticket', { ticket: ticket });
}

const ticket = urlParams.get('ticket');
const genericDetailsUnionUnparsed = redeem_ticket(ticket).then(genericDetailsUnion => {

  const years = genericDetailsUnion.years;
  const numberOfGenerics = genericDetailsUnion.counts;
  const details = genericDetailsUnion.details;

  Chart.defaults.backgroundColor = '#000000';
  Chart.defaults.borderColor = '#FFFFFF';
  Chart.defaults.borderWidth = 2;
  Chart.defaults.color = '#FFFFFF';

  const tbody = document.querySelector('tbody');

  details.forEach(detail => {
    const row = document.createElement('tr');

    const brandName = detail.brand_name || 'Unspecified';
    const activeIngredients = detail.active_ingredients ? detail.active_ingredients.map(ingredient => `Name: ${ingredient.name} & Strength: ${ingredient.strength}`).join(', ') : 'Unspecified';
    const dosageForm = detail.dosage_form || 'Unspecified';
    const productType = detail.product_type || 'Unspecified';

    row.innerHTML = `
              <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900 dark:text-white">${brandName}</td>
              <td class="whitespace-nowrap px-4 py-2 text-gray-700 dark:text-gray-200">${activeIngredients}</td>
              <td class="whitespace-nowrap px-4 py-2 text-gray-700 dark:text-gray-200">${dosageForm}</td>
              <td class="whitespace-nowrap px-4 py-2 text-gray-700 dark:text-gray-200">${productType}</td>
            `;

    tbody.appendChild(row);
  });

  const ctx = document.getElementById('genericsChart').getContext('2d');
  const chart = new Chart(ctx, {
    type: 'line',
    data: {
      labels: years,
      datasets: [{
        label: 'Number of Generics',
        data: numberOfGenerics,
      }]
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      plugins: {
        customCanvasBackgroundColor: {
          color: 'white',
        }
      },
      scales: {
        xAxes: [{
          type: 'time',
          time: {
            unit: 'year'
          },
        }],
        yAxes: [{
          ticks: {
            beginAtZero: true
          }
        }]
      }
    }
  });
});