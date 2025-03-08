import { useState, useEffect } from 'react';
import {
  Container,
  Paper,
  TextField,
  Button,
  Typography,
  Box,
  CircularProgress,
  List,
  ListItem,
  ListItemText,
  FormControlLabel,
  Switch,
  Slider,
  Alert,
  Chip,
  IconButton,
  Tooltip,
} from '@mui/material';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import RefreshIcon from '@mui/icons-material/Refresh';
import axios from 'axios';

const theme = createTheme({
  palette: {
    primary: {
      main: '#1976d2',
    },
    secondary: {
      main: '#dc004e',
    },
    success: {
      main: '#2e7d32',
    },
    error: {
      main: '#d32f2f',
    },
  },
});

interface CrawlResult {
  url: string;
  title: string;
  status: number;
}

interface ErrorResponse {
  error: string;
}

interface ServerStatus {
  status: string;
  version: string;
}

function App() {
  const [url, setUrl] = useState('');
  const [depth, setDepth] = useState(3);
  const [maxPages, setMaxPages] = useState(100);
  const [sameDomain, setSameDomain] = useState(true);
  const [isLoading, setIsLoading] = useState(false);
  const [results, setResults] = useState<CrawlResult[]>([]);
  const [error, setError] = useState('');
  const [serverStatus, setServerStatus] = useState<ServerStatus | null>(null);
  const [checkingStatus, setCheckingStatus] = useState(false);

  useEffect(() => {
    checkServerStatus();
  }, []);

  const checkServerStatus = async () => {
    setCheckingStatus(true);
    try {
      const response = await axios.get('http://localhost:8000/status');
      setServerStatus(response.data);
      setError('');
    } catch (err) {
      setServerStatus(null);
      setError('Server is offline. Please check your backend server.');
    } finally {
      setCheckingStatus(false);
    }
  };

  const handleCrawl = async () => {
    if (!url) {
      setError('Please enter a URL');
      return;
    }

    // Basic URL validation
    try {
      new URL(url);
    } catch {
      setError('Please enter a valid URL (e.g., https://example.com)');
      return;
    }

    setIsLoading(true);
    setError('');
    setResults([]);

    try {
      const response = await axios.post('http://localhost:8000/crawl', {
        start_url: url,
        depth_limit: depth,
        max_pages: maxPages,
        same_domain: sameDomain,
      }, {
        validateStatus: () => true, // Accept all status codes
      });

      // Check if the response contains data
      if (response.data && Array.isArray(response.data)) {
        setResults(response.data);
        
        if (response.data.length === 0) {
          setError('No pages were crawled. Check the URL and try again.');
        }
      } else {
        setError('Invalid response format from server.');
      }
    } catch (err) {
      if (axios.isAxiosError(err) && err.response?.data?.error) {
        setError(err.response.data.error);
      } else {
        setError('Failed to crawl. Please check the URL and try again.');
      }
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  };

  const getStatusColor = (status: number) => {
    if (status >= 200 && status < 300) return 'success';
    if (status >= 300 && status < 400) return 'primary';
    if (status >= 400 && status < 500) return 'warning';
    return 'error';
  };

  return (
    <ThemeProvider theme={theme}>
      <Container maxWidth="md" sx={{ py: 4 }}>
        <Paper elevation={3} sx={{ p: 4 }}>
          <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 2 }}>
            <Typography variant="h4" component="h1">
              Web Crawler
            </Typography>
            <Box sx={{ display: 'flex', alignItems: 'center' }}>
              {serverStatus && (
                <Chip 
                  label={`Server v${serverStatus.version}`} 
                  color="primary" 
                  size="small" 
                  sx={{ mr: 1 }}
                />
              )}
              <Tooltip title="Check server status">
                <IconButton onClick={checkServerStatus} disabled={checkingStatus}>
                  {checkingStatus ? <CircularProgress size={24} /> : <RefreshIcon />}
                </IconButton>
              </Tooltip>
            </Box>
          </Box>

          {error && (
            <Box sx={{ mt: 2, mb: 3 }}>
              <Alert severity="error">{error}</Alert>
            </Box>
          )}

          <Box component="form" sx={{ mt: 3 }}>
            <TextField
              fullWidth
              label="Start URL"
              variant="outlined"
              value={url}
              onChange={(e) => setUrl(e.target.value)}
              error={!!error && error.includes('URL')}
              placeholder="https://example.com"
              sx={{ mb: 3 }}
            />

            <Typography gutterBottom>Crawl Depth: {depth}</Typography>
            <Slider
              value={depth}
              onChange={(_, value) => setDepth(value as number)}
              min={1}
              max={10}
              marks
              valueLabelDisplay="auto"
              sx={{ mb: 3 }}
            />

            <Typography gutterBottom>Max Pages: {maxPages}</Typography>
            <Slider
              value={maxPages}
              onChange={(_, value) => setMaxPages(value as number)}
              min={10}
              max={500}
              step={10}
              marks
              valueLabelDisplay="auto"
              sx={{ mb: 3 }}
            />

            <FormControlLabel
              control={
                <Switch
                  checked={sameDomain}
                  onChange={(e) => setSameDomain(e.target.checked)}
                />
              }
              label="Stay on Same Domain"
              sx={{ mb: 3, display: 'block' }}
            />

            <Button
              variant="contained"
              onClick={handleCrawl}
              disabled={isLoading || !serverStatus}
              fullWidth
              size="large"
            >
              {isLoading ? <CircularProgress size={24} /> : 'Start Crawling'}
            </Button>
          </Box>

          {results.length > 0 && (
            <Box sx={{ mt: 4 }}>
              <Typography variant="h6" gutterBottom>
                Crawl Results ({results.length} pages)
              </Typography>
              <List>
                {results.map((result, index) => (
                  <ListItem
                    key={index}
                    divider
                    sx={{
                      display: 'flex',
                      flexDirection: 'column',
                      alignItems: 'flex-start',
                    }}
                  >
                    <Box sx={{ display: 'flex', alignItems: 'center', mb: 1 }}>
                      <Typography variant="h6" component="div">
                        {result.title}
                      </Typography>
                      <Chip
                        label={result.status}
                        color={getStatusColor(result.status)}
                        size="small"
                        sx={{ ml: 2 }}
                      />
                    </Box>
                    <Typography
                      variant="body2"
                      color="text.secondary"
                      component="div"
                      sx={{
                        wordBreak: 'break-all',
                      }}
                    >
                      {result.url}
                    </Typography>
                  </ListItem>
                ))}
              </List>
            </Box>
          )}
        </Paper>
      </Container>
    </ThemeProvider>
  );
}

export default App;
