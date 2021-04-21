const API_URL = process.env.NODE_ENV === 'production' ? window.location.origin : 'http://localhost:8000';

export {
  // eslint-disable-next-line import/prefer-default-export
  API_URL,
};
