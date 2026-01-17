export const formatKeyName = (key: string) => {
  return key
    .replaceAll('Key', '')
    .replaceAll('Left', '')
    .replaceAll('Right', '')
    .replaceAll('Control', 'Ctrl')
    .replaceAll('Meta', 'Cmd')
    .replaceAll('Alt', 'Option');
};
