import {
  Checkbox,
  FormControl,
  FormControlLabel,
  FormLabel,
  Paper,
  Radio,
  RadioGroup,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
} from '@mui/material';

type Row = [string, boolean, boolean, boolean];
const rows: Row[] = [
  ['Easy', false, false, false],
  ['Normal', true, false, false],
  ['Hard', true, true, false],
  ['Extreme', true, true, true],
];

function buildOnChangeInputElement(callback: (value: number) => void) {
  return (ev: React.ChangeEvent<HTMLInputElement>) => {
    callback(Number(ev.target.value));
  };
}

export default function Difficulty(props: {
  difficulty: number;
  onChange: (difficulty: number) => void;
}) {
  return (
    <FormControl style={{ width: '100%' }}>
      <FormLabel id=""></FormLabel>
      <RadioGroup
        name="radio-buttons-group"
        value={props.difficulty}
        onChange={buildOnChangeInputElement(props.onChange)}
      >
        <TableContainer component={Paper}>
          <Table sx={{ minWidth: 650 }} aria-label="simple table">
            <TableHead>
              <TableRow
                sx={{
                  'td, th': {
                    pt: 1,
                    pb: 1,
                  },
                }}
              >
                <TableCell />
                <TableCell
                  align="center"
                  title="Shuffle the ROM to be found in the Hand Scanner."
                >
                  Shuffle secret&nbsp;roms
                </TableCell>
                <TableCell
                  align="center"
                  title="You may need to use glitches to retrieve items."
                  style={{ fontSize: 0 }}
                >
                  Need glitches
                </TableCell>
                <TableCell
                  align="center"
                  title="Items such as Holy Grail and Game Master are also shuffled unconditionally."
                  style={{ fontSize: 0 }}
                >
                  Absolutely shuffle
                </TableCell>
              </TableRow>
            </TableHead>
            <TableBody>
              {rows
                .map((row, i) => [row, i] as [Row, number])
                .filter(([row, _i]) => row[0] && !row[2])
                .map(([row, i]) => (
                  <TableRow
                    key={row[0] as string}
                    sx={{
                      'td, th': {
                        pt: 1,
                        pb: 1,
                      },
                      '&:last-child td, &:last-child th': { border: 0 },
                    }}
                  >
                    <TableCell component="th" scope="row">
                      <FormControlLabel
                        value={i}
                        control={<Radio />}
                        label={row[0]}
                      />
                    </TableCell>
                    <TableCell align="center">
                      <Checkbox checked={row[1] as boolean} disabled={true} />
                    </TableCell>
                    <TableCell align="center">
                      <Checkbox
                        checked={row[2] as boolean}
                        disabled={true}
                        style={{ display: 'none' }}
                      />
                    </TableCell>
                    <TableCell align="center">
                      <Checkbox
                        checked={row[3] as boolean}
                        disabled={true}
                        hidden={true}
                        style={{ display: 'none' }}
                      />
                    </TableCell>
                  </TableRow>
                ))}
            </TableBody>
          </Table>
        </TableContainer>
      </RadioGroup>
    </FormControl>
  );
}
