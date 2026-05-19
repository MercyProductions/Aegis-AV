/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{ts,tsx}', './electron/**/*.{ts,tsx}'],
  theme: {
    extend: {
      colors: {
        aegis: {
          bg: '#05070A',
          panel: '#0B1016',
          card: '#101720',
          elevated: '#141D28',
          line: 'rgba(255,255,255,0.06)',
          green: '#6CFF6C',
          green2: '#34D058',
          red: '#FF4D4D',
          amber: '#FFB84D'
        }
      },
      fontFamily: {
        sans: ['Inter', 'Satoshi', 'SF Pro Display', 'ui-sans-serif', 'system-ui', 'sans-serif']
      },
      boxShadow: {
        glow: '0 0 42px rgba(108,255,108,0.18)',
        card: '0 24px 80px rgba(0,0,0,0.38)'
      },
      backgroundImage: {
        'aegis-radial': 'radial-gradient(circle at 50% 0%, rgba(108,255,108,0.12), transparent 34%)'
      },
      opacity: {
        15: '0.15',
        18: '0.18',
        35: '0.35',
        42: '0.42',
        45: '0.45',
        48: '0.48',
        52: '0.52',
        55: '0.55',
        56: '0.56',
        58: '0.58',
        62: '0.62',
        65: '0.65',
        68: '0.68',
        72: '0.72',
        78: '0.78',
        82: '0.82',
        96: '0.96'
      }
    }
  },
  plugins: []
};
